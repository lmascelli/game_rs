#[path = "./config.rs"]
mod config;
#[path = "./error.rs"]
mod error;
#[path = "./level.rs"]
mod level;
#[path = "./renderer.rs"]
mod renderer;

pub use config::{AppConfig, AppConfigManager};
pub use error::GameError;
pub use level::{Level, LevelBuilder};
pub use renderer::Renderer;

enum GameEvent {
    Tick,
}

pub struct Game {
    window: Option<winit::window::Window>,
    size: winit::dpi::PhysicalSize<u32>,
    title: String,
    renderer: Option<Renderer>,
    current_level_builder: Option<LevelBuilder>,
    current_level: Option<Level>,
    config: AppConfigManager,
}

impl Game {
    pub fn create(
        title: &str,
        config: AppConfigManager,
        starting_level_path: &str,
    ) -> Result<Self, GameError> {
        Ok(Self {
            window: None,
            renderer: None,
            title: title.to_string(),
            size: winit::dpi::PhysicalSize {
                width: config.window_width(),
                height: config.window_height(),
            },
            current_level_builder: Some(
                LevelBuilder::load_from_tmx_buffer(starting_level_path)
                    .expect("Failed to parse the level"),
            ),
            current_level: None,
            config,
        })
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        let event_loop = match winit::event_loop::EventLoop::<GameEvent>::with_user_event().build()
        {
            Ok(event_loop) => {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                event_loop
            }
            Err(err) => {
                eprintln!("ERROR: {err:?}");
                return Err(GameError::EventLoopError(err));
            }
        };

        let event_loop_proxy = event_loop.create_proxy();
        std::thread::spawn(move || {
            loop {
                let _unused_ret = event_loop_proxy.send_event(GameEvent::Tick);
                std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
            }
        });

        if let Err(err) = event_loop.run_app(self) {
            return Err(GameError::EventLoopError(err));
        }
        Ok(())
    }

    fn input(&mut self) -> bool {
        false
    }

    fn logic(&mut self) {}

    fn render(&mut self) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.render();
        }
    }
}

impl winit::application::ApplicationHandler<GameEvent> for Game {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            self.window.replace(
                event_loop
                    .create_window(
                        winit::window::Window::default_attributes()
                            .with_title(self.title.clone())
                            .with_inner_size(self.size),
                    )
                    .expect("Failed to create the window"),
            );
            if self.renderer.is_none() {
                self.renderer.replace(pollster::block_on(Renderer::new(
                    &self.window.as_ref().unwrap(),
                    &self.config,
                )));
                if let Some(_) = self.current_level_builder {
                    let level_builder = self.current_level_builder.take().unwrap();
                    self.current_level
                        .replace(level_builder.build(self.renderer.as_mut().unwrap()));
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent;
        if self.input() {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                self.renderer.take();
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }
    
    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: GameEvent) {
        match event {
            GameEvent::Tick => {
                self.logic();
                if self.window.is_some() && self.renderer.is_some() {
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
        }
    }
}

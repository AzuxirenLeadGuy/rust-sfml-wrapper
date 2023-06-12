use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::{ContextSettings, Event, Style, VideoMode},
    SfBox,
};

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub style: Style,
    pub context_settings: ContextSettings,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: String::from("SFML Rust game"),
            size: (800, 600),
            style: Style::DEFAULT,
            context_settings: ContextSettings::default(),
        }
    }
}
pub enum UpdateResult<ScreenEnumType> {
    NoChange,
    Change(ScreenEnumType),
    ExitGame,
}
pub trait ScreenEnum<GameConstants, ScreenEnumType>: Default {
    fn init(&mut self, constants: &GameConstants) -> u8;
    fn update(
        &mut self,
        constants: &mut GameConstants,
        delta_time_ms: i32,
    ) -> UpdateResult<ScreenEnumType>;
    fn draw(&self, window: &mut RenderWindow) -> u8;
    fn background_color(&self) -> Color;
}

pub struct CoreSfmlGameEnum<GameConstants, ScreenEnumType>
where
    ScreenEnumType: ScreenEnum<GameConstants, ScreenEnumType>,
{
    pub clock: SfBox<Clock>,
    pub running_screen: ScreenEnumType,
    pub load_screen: ScreenEnumType,
    pub is_loading: bool,
    pub window: RenderWindow,
    pub settings: GameConstants,
}

impl<GameConstants, ScreenEnumType> CoreSfmlGameEnum<GameConstants, ScreenEnumType>
where
    ScreenEnumType: ScreenEnum<GameConstants, ScreenEnumType>,
{
    async fn screen_loader(&mut self) {
        self.running_screen.init(&self.settings);
        self.is_loading = false;
    }
    pub fn exit(&mut self) {
        self.window.close();
    }
    fn run_frame(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.exit();
            }
        }
        let delta_time_ms = self.clock.restart().as_milliseconds();
        let cur_screen = match self.is_loading {
            true => &mut self.load_screen,
            false => &mut self.running_screen,
        };
        let change_screen = cur_screen.update(&mut self.settings, delta_time_ms);
        self.window.clear(cur_screen.background_color());
        cur_screen.draw(&mut self.window);
        self.window.display();
        match change_screen {
            UpdateResult::ExitGame => self.exit(),
            UpdateResult::NoChange => (),
            UpdateResult::Change(x) => {
                self.is_loading = true;
                self.running_screen = x;
                _ = self.screen_loader();
            }
        }
    }
    pub fn run_game(&mut self) {
        while self.window.is_open() {
            self.run_frame();
        }
    }
}

pub fn create_sfml_game_object<GameConstants, ScreenEnumType>(
    window_settings: WindowSettings,
    settings: GameConstants,
    running_screen: ScreenEnumType,
    load_screen: ScreenEnumType,
) -> CoreSfmlGameEnum<GameConstants, ScreenEnumType>
where
    ScreenEnumType: ScreenEnum<GameConstants, ScreenEnumType>,
{
    let mut running_screen = running_screen;
    running_screen.init(&settings);
    let mut load_screen = load_screen;
    load_screen.init(&settings);

    CoreSfmlGameEnum {
        clock: Clock::start(),
        is_loading: false,
        window: RenderWindow::new(
            VideoMode::new(window_settings.size.0, window_settings.size.1, 32),
            &window_settings.title,
            window_settings.style,
            &window_settings.context_settings,
        ),
        running_screen,
        load_screen,
        settings,
    }
}

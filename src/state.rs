use agb::input::ButtonController;

//global state which needs to persist as long as the game runs
//pulled from save data if found, or sets defaults
pub struct GameState {
    pub input: agb::input::ButtonController,
    pub game_settings: SettingsState,
    pub game_progress: u16,
    pub game_state: TopLevelState,
}
impl GameState {
    pub fn setup() -> GameState {
        //things which don't change based on save state
        let input = ButtonController::new();
        //if a save is detected

        /*placeholder code. I'll likely be asking agb's save/EEPROM API something like "does valid data exist here," which returns a bool or Option you'd match on — a good next rabbit hole*/
        let save = false;
        if save {
            Self::from_save(input)
        } else {
            Self::new(input)
        }
    }
    fn new(input: ButtonController) -> GameState {
        GameState {
            input: (input),
            game_settings: SettingsState::new(),
            game_progress: (0),
            game_state: TopLevelState::MainMenu(MainMenuState {}),
        }
    }
    fn from_save(input: ButtonController) -> GameState {
        GameState {
            input: (input),
            game_settings: (SettingsState::from_save()),
            game_progress: (0/*load progress from save file*/),
            game_state: TopLevelState::MainMenu(MainMenuState {}),
        }
    }
}

pub struct SettingsState {
    //text_speed, words per minute, sound volume, enable flashing, tone picker (preset1..x or custom), pitch, noise, filtering, etc.
}
impl SettingsState {
    fn new() -> SettingsState {
        SettingsState {}
    }
    //should interact with the agb::save module to read from saves
    fn from_save() -> SettingsState {
        SettingsState {}
    }

    fn save_settings(&self) {}
}

pub enum TopLevelState {
    MainMenu(MainMenuState),
    Settings(SettingsMenuState),
    InGame(InGameState),
}

pub struct MainMenuState {}
impl MainMenuState {
    fn update(&self) {}
    fn draw(&self) {}
}

pub struct SettingsMenuState {
    from: SettingsFrom,
}
impl SettingsMenuState {
    fn update(&self) {}
    fn draw(&self) {}
}
enum SettingsFrom {
    TopLevel,
    InGame,
}

pub struct InGameState {
    in_game_menu: InGameMenuState,
    location: LocationState,
}
impl InGameState {
    fn new() -> InGameState {
        InGameState {
            in_game_menu: InGameMenuState::None,
            location: (LocationState::RoomN),
        }
    }
    fn update(&self) {}
    fn draw(&self) {}
}

enum InGameMenuState {
    None,
    Settings(SettingsMenuState),
    InDictionary(InDictionary),
    InTapper(InTapper),
    InPuzzle(InPuzzle),
}

enum LocationState {
    RoomW,
    RoomN,
    RoomE,
    RoomS,
    Outside,
}

struct InDictionary {}

struct InTapper {
    framecounter: usize,
}

struct InPuzzle {}

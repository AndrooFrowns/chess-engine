use std::os::linux::raw::time_t;

enum DebugState { ON, OFF }

enum CopyAndRegisterationState { CHECKING, OKAY, ERROR }

enum ScoreField {
    LowerBoundCentiPawn(f64),
    UpperBoundCentiPawn(f64),
    LowerBoundMate(isize),
    UpperBoundMate(isize),
}

enum OptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

struct OptionField {
    name: String,
    kind: OptionType,
    default: Option<String>,
    max: Option<String>,
    min: Option<String>,
    var: Option<String>,
}

enum InfoField {
    Depth(isize),
    Seldepth(isize),
    Time(std::time::Duration),
    Nodes(isize),
    Pv(Vec<Movement>),
    Multipv(isize),
    Score(ScoreField),
    Currmove(isize),
    Currmovenumber(isize),
    /// Current move number, should be 1 for the first move, not 0
    Hashfull(isize),
    Nps(f64),
    /// nodes per second searched, should be sent regularly
    Tbhits(isize),
    Sbhits(isize),
    Cpuload(f64),
    String(String),
    Refutation(Vec<Movement>),
    Currline(Vec<Movement>),
    Option(OptionField),
}


enum ReceiveMessage {
    /// Initial command from the GUI to indicate UCI will be used
    Uci,
    /// Indicates whether the debug info should be sent to the GUI.
    Debug(DebugState),
    /// Used for the GUI to sync with the Engine
    IsReady,
    /// Allows the GUI to set on option on the Engine
    SetOption { name: String, value: String },
    /// Allows communicating in order to register the engine
    Register { tokens: String },
    /// Used for the GUI to indicate the next position is from a new game. The engine should not 100% rely on this being sent
    UciNewGame,
    /// Used to convey the gamestate to the engine
    Position(GameState),
    /// Tells the Engine to search for the best move with potentially some constraints
    Go(GoInfo),
    /// Tells the engine to stop search for better moves, and respond with bestmove or possibly ponder
    Stop,
    /// the user has played the expected move. This will be sent if the engine was told to ponder on the same move the user has played. The engine should continue searching but switch from pondering to normal search.
    Ponderhit,
    /// tells the engine to close safely as soon as possible
    Quit,
}

enum SendMessage {
    /// Used to identify the engine in response to `Uci`
    Id { name: String, author: String },
    /// Sent after `Id` and all the available optional options to indicate the engine is ready
    UciOk,
    /// Response to `IsReady` indicates that the engine has processed all inputs and is ready to accept new commands
    ReadyOk,
    /// Indicates to the GUI what the selected move and what the engine would like to ponder.
    /// Directly before this the engine should send a final info command so the GUI has the information it needs
    Bestmove { selection: Movement, ponder: Option<Movement> },
    /// Used to protect from copying, `CHECKING` should be sent before the correct other result
    CopyProtection(CopyRegisterationState),
    /// registers the engine
    Registration(CopyRegisterationState),
    /// This should be used to share information with the GUI whenever the requested fields change
    Info(Vec<InfoField>),
}






//Receiver - the tv

pub struct Television {
    volume: i32,
    muted: bool,
    channel: i32,
}

impl Television {
    pub fn new() -> Self {
        Self { volume: 10, muted: false, channel: 1 }
    }

    pub fn show(&self) {
        println!(" TV -> volume: {} muted: {} channel: {}",
                    self.volume, self.muted, self.channel);
    }
}


//comment trait
pub trait Command {

    fn execute(&mut self);
    fn undo(&mut self);
    fn name(&self) -> &str;
    
}

//button 1 volume 
pub struct VolumeUpCommand {
    tv: std::rc::Rc<std::cell::RefCell<Television>>,
    amount: i32,
}

impl VolumeUpCommand {
    pub fn new(
        tv: std::rc::Rc<std::cell::RefCell<Television>>,
        amount: i32,
    ) -> Self{
        Self { tv, amount }
    }
}

impl Command for VolumeUpCommand {
    fn execute(&mut self) {
        self.tv.borrow_mut().volume += self.amount;

    }
    fn undo(&mut self) {
        self.tv.borrow_mut().volume -= self.amount;
    }
    fn name(&self) -> &str {
        "VolumeUp"
    }
}


//button 2 mute
pub struct MuteCommand {
    tv: std::rc::Rc<std::cell::RefCell<Television>>,
}

impl MuteCommand {
    pub fn new(tv: std::rc::Rc<std::cell::RefCell<Television>>) -> Self{
        Self { tv }
    }
}

impl Command for MuteCommand {
    fn execute(&mut self) {
        self.tv.borrow_mut().muted = true;
    }
    fn undo(&mut self) {
        self.tv.borrow_mut().muted = false;
    }
    fn name(&self) -> &str {
        "Mute"
    }
}


//button 3 change channel
pub struct ChannelCommand {
    tv: std::rc::Rc<std::cell::RefCell<Television>>,
    new_channel: i32,
    prev_channel: i32,
}

impl ChannelCommand {
    pub fn new(
        tv: std::rc::Rc<std::cell::RefCell<Television>>,
        channel: i32,
    ) -> Self {
        Self { tv, new_channel: channel, prev_channel: 0 }
    }
}


impl Command for ChannelCommand {
    fn execute(&mut self) {
        let mut tv = self.tv.borrow_mut();
        self.prev_channel = tv.channel;
        tv.channel = self.new_channel;
    }

    fn undo(&mut self) {
        self.tv.borrow_mut().channel = self.prev_channel;
    }
    fn name(&self) -> &str { "Channel"}
}

//Remote - the invoker
pub struct Remote {
    history: Vec<Box<dyn Command>>,
}

impl Remote {
   pub fn new() -> Self {
        Self { history: vec![] }
    }

    pub fn press(&mut self, mut cmd: Box<dyn Command>){
        println!(" [press] {}", cmd.name());
        cmd.execute();
        self.history.push(cmd);
    }

    pub fn undo_last(&mut self){
        match self.history.pop() {
            Some(mut cmd) => {
                println!(" [undo] {}", cmd.name());
                cmd.undo();
            }
            None => println!(" Noting to undo"),
        }
    }
}

pub struct JavaContext {
    env: *mut JNIEnv,
}

pub struct Sequencer<'lib> {
    java_context: Arc<JavaContext>,
    lib:         Arc<AmidiLibrary>,
    dev:         Arc<MidiDevice<'lib>>,
    input_port:  Arc<MidiInputPort<'lib>>,
    output_port: Arc<MidiOutputPort<'lib>>,
    //etc
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
    stuff goes here
}

impl<'lib> Sequencer<'lib> {

    pub async fn new() -> Result<Self,MyError> {
        let lib = todo!();
        let dev = todo!();
        let input_port = todo!();
        let output_port = todo!();
        Self {
            lib,
            dev,
            input_port,
            output_port,
        }
    }

    pub async fn do_stuff(&self) {
        todo!();
    }

    pub async fn trigger_pad(&self,pad_idx: usize) {
        todo!();
    }

    pub async fn add_note(&self,note_idx: usize) {
        todo!();
    }
}

let sequencer = Sequencer::new()?;

sequencer.do_stuff().await?;
sequencer.trigger_pad().await?;
sequencer.add_note().await?;
sequencer.add_clip().await?;
sequencer.reverse_sample().await?;

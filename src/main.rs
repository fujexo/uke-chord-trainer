use std::time::Duration;
use rand::seq::SliceRandom;
use yew::services::ConsoleService;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{html, App, Component, ComponentLink, Html, ShouldRender};

mod bindings;

static CHORDS: [&str; 42] = [
    "C",     "D",     "E",     "F",     "G",     "A",     "B",
    "C7",    "D7",    "E7",    "F7",    "G7",    "A7",    "B7",
    "Cm",    "Dm",    "Em",    "Fm",    "Gm",    "Am",    "Bm",
    "Cm7",   "Dm7",   "Em7",   "Fm7",   "Gm7",   "Am7",   "Bm7",
    //"Cdim",  "Ddim",  "Edim",  "Fdim",  "Gdim",  "Admin", "Bdim",
    //"Caug",  "Daug",  "Eaug",  "Faug",  "Gaug",  "Aaug",  "Baug",
    "C6",    "D6",    "E6",    "F6",    "G6",    "A6",    "B6",
    "Cmaj7", "Dmaj7", "Emaj7", "Fmaj7", "Gmaj7", "Amaj7", "Bmaj7",
    // "C9",    "D9",    "E9",    "F9",    "G9",    "A9",    "B9",
];

// "Db", "Db7", "Dbm", "Dbm7", "Dbdim", "Dbaug", "Db6", "Dbmaj7", "Db9",
// "Eb", "Eb7", "Ebm", "Ebm7", "EbEim", "Ebaug", "Eb6", "Ebmaj7", "Eb9",
// "Gb", "Gb7", "Gbm", "Gbm7", "Gbdim", "Gbaug", "Gb6", "Gbmaj7", "Gb9",
// "Ab", "Ab7", "Abm", "Abm7", "Abdim", "Abaug", "Ab6", "Abmaj7", "Ab9",
// "Bb", "Bb7", "Bbm", "Bbm7", "Bbdim", "Bbaug", "Bb6", "Bbmaj7", "Bb9",

pub enum Msg {
    IncrementSeconds,
    DecrementSeconds,
    ToggleChord(String),
    ToggleChordImage,
    ToggleMetronome,
    Tick,
}

pub struct Model {
    link: ComponentLink<Self>,
    timer: f64,
    current_chord: String,
    chords: Vec<String>,
    chordimage: bool,
    metronome: bool,
    _clock: IntervalTask,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let clock_handle =
            IntervalService::spawn(Duration::from_secs(2), link.callback(|_| Msg::Tick));

        Self {
            link,
            timer: 2.0,
            chords: vec!["C".to_string(), "F".to_string(), "G7".to_string(), "Am".to_string(), "G".to_string(), "D".to_string(), "A".to_string(),],
            current_chord: "C".to_string(),
            chordimage: false,
            metronome: false,
            _clock: clock_handle,
        }

    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementSeconds => {
                self.timer += 0.25;
                self._clock = IntervalService::spawn(Duration::from_millis((self.timer * 1000.0) as u64), self.link.callback(|_| Msg::Tick));
                true
            }
            Msg::DecrementSeconds => {
                self.timer -= 0.25;
                self._clock = IntervalService::spawn(Duration::from_millis((self.timer * 1000.0) as u64), self.link.callback(|_| Msg::Tick));
                true
            }
            Msg::ToggleChord(chord) => {
                if self.chords.contains(&chord) {
                    self.chords.remove(self.chords.iter().position(|x| *x == chord).unwrap());
                } else {
                    self.chords.push(chord)
                }
                ConsoleService::log(&*format!("Activated Chords: {:?}", self.chords));
                true
            }
            Msg::ToggleChordImage => {
                self.chordimage = !self.chordimage;
                true
            }
            Msg::ToggleMetronome => {
                self.metronome = !self.metronome;
                true
            }
            Msg::Tick => {
                self.current_chord = self.chords.choose(&mut rand::thread_rng()).unwrap().clone();
                ConsoleService::log(&*format!("New Chord: {:?}", self.current_chord));

                if self.metronome == true {
                    bindings::play();
                }

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        let chord_image_path = "assets/img/".to_string() + &self.current_chord + ".svg";

        html! {
            <div>

                <div class="chorddisplay">
                    <span class="chordtext"> { self.current_chord.clone() }</span>
                    // <span class="chordimage"> { "IMG" } </span>

                    { if self.chordimage == true {
                        html!{
                            <img class="chordimage" src=chord_image_path />
                        } 
                      } else {
                          html!{}
                      }
                    }
                    
                </div>
                <div class="chordsettings">
                    <nav class="menu">
                        { for CHORDS.map(|name| if self.chords.contains(&name.to_string()) {
                            html!{ <button class="button chordselector is-active" onclick=self.link.callback(move |_| Msg::ToggleChord(name.to_string()))> { name } </button> }
                        } else {
                            html!{ <button class="button chordselector" onclick=self.link.callback(move |_| Msg::ToggleChord(name.to_string()))> { name } </button> }
                        }) }
                    </nav>
                </div>
                <div class="timersettings">
                    <nav class="menu">
                        <p>{ "Time: " }
                            <span class="timer">{ self.timer }</span>
                            { " Sec." }
                        </p>
                        <button class="button timerselect" onclick=self.link.callback(|_| Msg::DecrementSeconds)> { "Decrease Time" }</button>
                        <button class="button timerselect" onclick=self.link.callback(|_| Msg::IncrementSeconds)> { "Increase Time" } </button>
                    </nav>
                </div>
                <div class="settings">
                <nav class="menu">
                    { if self.chordimage == true {
                        html!{ <button class="button is-active" onclick=self.link.callback(move |_| Msg::ToggleChordImage)> { "Toggle Chord Image" } </button> }
                    } else {
                        html!{ <button class="button" onclick=self.link.callback(move |_| Msg::ToggleChordImage)> { "Toggle Chord Image" } </button> }
                    }}
                    { if self.metronome == true {
                        html!{ <button class="button is-active" onclick=self.link.callback(move |_| Msg::ToggleMetronome)> { "Toggle Metronome" } </button> }
                    } else {
                        html!{ <button class="button" onclick=self.link.callback(move |_| Msg::ToggleMetronome)> { "Toggle Metronome" } </button> }
                    }}
                </nav>
            </div>
                <audio id="audioplayer" muted=true >
                    <source src="assets/audio/1.mp3" type="audio/mpeg" />
                    { "Your browser does not support the audio element." }
                </audio>
            </div>
        }
    }
}

fn mount_app(selector: &'static str, app: App<Model>) -> ComponentLink<Model> {
    let document = yew::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();
    let application = App::new();
    mount_app(".main", application);
    yew::run_loop();
}

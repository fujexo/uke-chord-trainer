use gloo_console::log;
use gloo_timers::callback::Interval;
use rand::seq::SliceRandom;
use std::str::FromStr;
use tera::{Context as TeraContext, Tera};
use ukebox::{Chord, Tuning, VoicingConfig};
use yew::prelude::*;
// use base64::encode;

mod bindings;

static CHORDS: [&str; 42] = [
    "C", "D", "E", "F", "G", "A", "B", "C7", "D7", "E7", "F7", "G7", "A7", "B7", "Cm", "Dm", "Em",
    "Fm", "Gm", "Am", "Bm", "Cm7", "Dm7", "Em7", "Fm7", "Gm7", "Am7", "Bm7",
    //"Cdim",  "Ddim",  "Edim",  "Fdim",  "Gdim",  "Admin", "Bdim",
    //"Caug",  "Daug",  "Eaug",  "Faug",  "Gaug",  "Aaug",  "Baug",
    "C6", "D6", "E6", "F6", "G6", "A6", "B6", "Cmaj7", "Dmaj7", "Emaj7", "Fmaj7", "Gmaj7", "Amaj7",
    "Bmaj7",
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
    timer: f64,
    current_chord: String,
    chords: Vec<String>,
    chordimage: bool,
    metronome: bool,
    _clock: Option<Interval>,
}

impl Model {
    fn cancel_timer(&mut self) {
        self._clock = None;
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    //fn create(_ctx: &Context<Self>, _props: Self::Properties, link: ComponentLink<Self>) -> Self {
    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let clock_handle = Interval::new(2_000, move || link.send_message(Msg::Tick));

        Self {
            timer: 2.0,
            chords: vec![
                "C".to_string(),
                "F".to_string(),
                "G7".to_string(),
                "Am".to_string(),
                "G".to_string(),
                "D".to_string(),
                "A".to_string(),
            ],
            current_chord: "C".to_string(),
            chordimage: false,
            metronome: false,
            _clock: Some(clock_handle),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::IncrementSeconds => {
                self.timer += 0.25;
                self.cancel_timer();
                self._clock = Some(Interval::new((self.timer * 1000.0) as u32, move || {
                    link.send_message(Msg::Tick);
                }));
                true
            }
            Msg::DecrementSeconds => {
                if self.timer >= 0.5 {
                    self.timer -= 0.25;
                }
                self.cancel_timer();
                self._clock = Some(Interval::new((self.timer * 1000.0) as u32, move || {
                    link.send_message(Msg::Tick);
                }));
                true
            }
            Msg::ToggleChord(chord) => {
                if self.chords.contains(&chord) {
                    self.chords
                        .remove(self.chords.iter().position(|x| *x == chord).unwrap());
                } else {
                    self.chords.push(chord)
                }
                log!(format!("Activated Chords: {:?}", self.chords));
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
                self.current_chord = match self.chords.choose(&mut rand::thread_rng()) {
                    Some(chord) => chord.clone(),
                    None => self.current_chord.clone(),
                };

                if self.metronome {
                    bindings::play();
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>

            <div class="chorddisplay">
            <span class="chordtext"> { self.current_chord.clone() }</span>

            {
                if self.chordimage {
                    let chord_image = chord_image(&self.current_chord).to_string();
                    let img = Html::from_html_unchecked(AttrValue::from(chord_image));

                    html!{
                        {img}
                    }
                } else {
                    html!{}
                }
            }

                </div>
                <div class="chordsettings">
                    <nav class="menu">
                        { for CHORDS.map(|name| if self.chords.contains(&name.to_string()) {
                            html!{ <button class="button chordselector is-active" onclick={ctx.link().callback(move |_| Msg::ToggleChord(name.to_string()))}> { name } </button> }
                        } else {
                            html!{ <button class="button chordselector" onclick={ctx.link().callback(move |_| Msg::ToggleChord(name.to_string()))}> { name } </button> }
                        }) }
                    </nav>
                </div>
                <div class="timersettings">
                    <nav class="menu">
                        <p>{ "Time: " }
                            <span class="timer">{ self.timer }</span>
                            { " Sec." }
                        </p>
                        <button class="button timerselect" onclick={ctx.link().callback(|_| Msg::DecrementSeconds)}> { "Decrease Time" }</button>
                        <button class="button timerselect" onclick={ctx.link().callback(|_| Msg::IncrementSeconds)}> { "Increase Time" } </button>
                    </nav>
                </div>
                <div class="settings">
                <nav class="menu">
                    { if self.chordimage {
                        html!{ <button class="button is-active" onclick={ctx.link().callback(move |_| Msg::ToggleChordImage)}> { "Toggle Chord Image" } </button> }
                    } else {
                        html!{ <button class="button" onclick={ctx.link().callback(move |_| Msg::ToggleChordImage)}> { "Toggle Chord Image" } </button> }
                    }}
                    { if self.metronome {
                        html!{ <button class="button is-active" onclick={ctx.link().callback(move |_| Msg::ToggleMetronome)}> { "Toggle Metronome" } </button> }
                    } else {
                        html!{ <button class="button" onclick={ctx.link().callback(move |_| Msg::ToggleMetronome)}> { "Toggle Metronome" } </button> }
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

fn main() {
    let document = gloo_utils::document();
    let element = document.query_selector(".main").unwrap().unwrap();

    yew::Renderer::<Model>::with_root(element).render();
}

fn svg_draw(finger: &str, string: i32, fret: i32, string_space: &i32) -> String {
    if fret <= 0 {
        return String::new();
    }

    let x = 50 + (string * string_space);
    let y = 50 + (fret * string_space);
    let radius = 13;

    let finger = format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"16\" fill=\"white\" font-weight=\"400\">{}</text>",
        x, y + 2, finger
    );

    let fret = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" />", x, y, radius);

    fret + &finger
}

fn chord_image(chord: &str) -> String {
    const STRING_SPACE: i32 = 40;

    let chord = Chord::from_str(chord).unwrap();

    let tuning = Tuning::C;

    let config = VoicingConfig {
        tuning,
        ..Default::default()
    };
    let voicing = chord.voicings(config).next().unwrap();
    let fingers = voicing.fingers_on_strings();

    let mut svg_add = String::new();

    for (i, fret) in voicing.frets().enumerate() {
        let ret = svg_draw(
            &fingers[i].to_string(),
            i as i32,
            fret as i32,
            &STRING_SPACE,
        );
        svg_add += &ret;
    }

    let mut context = TeraContext::new();
    context.insert("fingers", &svg_add);

    Tera::one_off(include_str!("../templates/chord.svg"), &context, false).unwrap()
}

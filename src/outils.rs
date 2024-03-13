pub mod ui {
    const ARRAY_OF_NOTES: [(&str, i32, &str, &str); 17] = [
    ("C", 0, "255, 255, 0", "🟡"),
    ("C#", 1, "0, 255, 0", "🟢"),
    ("Db", 1, "0, 255, 0", "🟢"),
    ("D", 2, "0, 128, 0", "🟩"),
    ("Eb", 3, "0, 255, 255", "🟦"),
    ("D#", 3, "0, 255, 255", "🟦"),
    ("E", 4, "0, 0, 255", "🔷"),
    ("F", 5, "0, 0, 128", "🔵"),
    ("F#", 6, "75, 0, 130 ", "🔵"),
    ("Gb", 6, "75, 0, 130 ", "🔵"),
    ("G", 7, "148, 0, 211", "🟣"),
    ("Ab", 8, "128, 0, 128", "🟪"),
    ("G#", 8, "128, 0, 128", "🟪"),
    ("A", 9, "139, 69, 19", "🔴"),
    ("Bb", 10, "255, 0, 0", "🟥️"),
    ("A#", 10, "255, 0, 0", "🟥️"),
    ("B", 11, "255, 165, 0", "🟠"),
];

const ARRAY_OF_GRAPH_NOTES: [(&str, i32); 12] = [
    ("white e", 1),
    ("black cs", 2),
    ("white d", 3),
    ("black ds", 4),
    ("white c", 5),
    ("white b", 6),
    ("black as", 7),
    ("white a", 8),
    ("black gs", 9),
    ("white g", 10),
    ("black fs", 11),
    ("white f", 12),
];

const ARRAY_OF_VECTORS: [(&str, [i32; 6]); 93] = [
    (" ", [0, 4, 7, 12, 16, 19]),
    ("ot", [0, 0, 0, 0, 0, 0]),
    ("min", [0, 3, 7, 12, 19, 24]),
    ("-", [0, 3, 7, 12, 19, 24]),
    ("dim", [0, 3, 6, 12, 19, 24]),
    ("(b5)", [0, 4, 6, 12, 19, 24]),
    ("aug", [0, 4, 8, 12, 19, 24]),
    ("5", [0, 7, 12, 19, 24, 31]),
    ("no5", [0, 4, 12, 16, 19, 24]),
    ("omit5", [0, 4, 12, 16, 19, 24]),
    ("m(no5)", [0, 3, 12, 15, 19, 24]),
    ("m(omit5)", [0, 3, 12, 15, 19, 24]),
    ("maj", [0, 4, 7, 12, 19, 24]),
    ("m", [0, 3, 7, 12, 19, 24]),
    ("sus2", [0, 2, 7, 12, 19, 24]),
    ("sus4", [0, 5, 7, 12, 19, 24]),
    ("sus", [0, 5, 7, 12, 19, 24]),
    ("6", [0, 4, 7, 9, 19, 24]),
    ("7", [0, 4, 7, 10, 16, 19]),
    ("7-5", [0, 4, 6, 10, 16, 18]),
    ("7b5", [0, 4, 6, 10, 16, 18]),
    ("7+5", [0, 4, 8, 10, 16, 20]),
    ("7#5", [0, 4, 8, 10, 16, 20]),
    ("7sus4", [0, 5, 7, 10, 19, 24]),
    ("m6", [0, 3, 7, 9, 19, 24]),
    ("m7", [0, 3, 7, 10, 19, 24]),
    ("m7-5", [0, 3, 6, 10, 15, 18]),
    ("m7b5", [0, 3, 6, 10, 15, 18]),
    ("m7+5", [0, 3, 8, 10, 19, 24]),
    ("m7#5", [0, 3, 8, 10, 19, 24]),
    ("dim6", [0, 3, 6, 8, 19, 24]),
    ("dim7", [0, 3, 6, 9, 19, 24]),
    ("7alt", [0, 3, 6, 9, 19, 24]),
    ("M7", [0, 4, 7, 11, 19, 24]),
    ("maj7", [0, 4, 7, 11, 19, 24]),
    ("M7+5", [0, 4, 8, 11, 19, 24]),
    ("mM7", [0, 3, 7, 11, 15, 19]),
    ("add4", [0, 4, 5, 7, 19, 24]),
    ("Madd4", [0, 4, 5, 7, 19, 24]),
    ("madd4", [0, 3, 5, 7, 19, 24]),
    ("add9", [0, 4, 7, 14, 19, 24]),
    ("Madd9", [0, 4, 7, 14, 19, 24]),
    ("madd9", [0, 3, 7, 14, 19, 24]),
    ("sus4add9", [0, 5, 7, 14, 19, 24]),
    ("sus4add2", [0, 2, 5, 7, 19, 24]),
    ("2", [0, 4, 7, 14, 19, 24]),
    ("add11", [0, 4, 7, 17, 19, 24]),
    ("m11", [0, 3, 5, 17, 19, 24]),
    ("4", [0, 4, 7, 17, 19, 24]),
    ("m69", [0, 3, 7, 9, 14, 24]),
    ("69", [0, 4, 7, 9, 14, 24]),
    ("9", [0, 4, 7, 10, 14, 24]),
    ("m9", [0, 3, 7, 10, 14, 24]),
    ("M9", [0, 4, 7, 11, 14, 24]),
    ("maj9", [0, 4, 7, 11, 14, 24]),
    ("9sus4", [0, 5, 7, 10, 14, 24]),
    ("7-9", [0, 4, 7, 10, 13, 24]),
    ("7b9", [0, 4, 7, 10, 13, 24]),
    ("7+9", [0, 4, 7, 10, 15, 24]),
    ("7#9", [0, 4, 7, 10, 15, 24]),
    ("9-5", [0, 4, 6, 10, 14, 24]),
    ("9b5", [0, 4, 6, 10, 14, 24]),
    ("9+5", [0, 4, 8, 10, 14, 24]),
    ("9#5", [0, 4, 8, 10, 14, 24]),
    ("7#9b5", [0, 4, 6, 10, 15, 24]),
    ("7#9#5", [0, 4, 8, 10, 15, 24]),
    ("m7b9b5", [0, 3, 6, 10, 13, 24]),
    ("7b9b5", [0, 4, 6, 10, 13, 24]),
    ("7b9#5", [0, 4, 8, 10, 13, 24]),
    ("11", [0, 7, 10, 14, 17, 24]),
    ("7+11", [0, 4, 7, 10, 18, 24]),
    ("7#11", [0, 4, 7, 10, 18, 24]),
    ("M7+11", [0, 4, 7, 11, 18, 24]),
    ("M7#11", [0, 4, 7, 11, 18, 24]),
    ("7b9#9", [0, 4, 7, 10, 13, 15]),
    ("7b9#11", [0, 4, 7, 10, 13, 18]),
    ("7#9#11", [0, 4, 7, 10, 15, 18]),
    ("7-13", [0, 4, 7, 10, 20, 24]),
    ("7b13", [0, 4, 7, 10, 20, 24]),
    ("m7add11", [0, 3, 7, 10, 17, 24]),
    ("M7add11", [0, 4, 7, 11, 17, 24]),
    ("mM7add11", [0, 3, 7, 11, 17, 24]),
    ("7b9b13", [0, 4, 7, 10, 13, 17]),
    ("9+11", [0, 4, 7, 10, 14, 18]),
    ("9#11", [0, 4, 7, 10, 14, 18]),
    ("13", [0, 4, 7, 10, 14, 21]),
    ("13-9", [0, 4, 7, 10, 13, 21]),
    ("13b9", [0, 4, 7, 10, 13, 21]),
    ("13+9", [0, 4, 7, 10, 15, 21]),
    ("13#9", [0, 4, 7, 10, 15, 21]),
    ("13+11", [0, 4, 7, 10, 18, 21]),
    ("13#11", [0, 4, 7, 10, 18, 21]),
    ("M7add13", [0, 4, 7, 9, 11, 14]),
];


fn create_octave(nombre: i32) -> yew::Html {
    let resultat = yew::html!(
        for ARRAY_OF_GRAPH_NOTES.iter().map(|&element|  render_octave(element.0,(element.1+12*nombre).try_into().unwrap()))
    );
    resultat
}

#[allow(unused)]
fn concatenate_octave(nombre: i32) -> yew::Html {
    let mut resultat: yew::virtual_dom::VNode = yew::html!();
    for octave in 0..nombre {
        resultat = yew::html!({ create_octave(octave) });
    }
    resultat
}

#[yew::function_component(AffichePiano)]
pub fn display_piano() -> yew::Html {
    let piano = yew::html!(
        <section>
            <div>
                <ul class="set">
                    <div class="notes">
                        <octave>
                            {create_octave(0)}
                            {create_octave(1)}
                            {create_octave(2)}
                        </octave>
                    </div>
                </ul>
            </div>
        </section>
    );
    piano
}

#[yew::function_component(SelecteurAccords)]
pub fn app() -> yew::Html {
    let list_options = ARRAY_OF_VECTORS.map(|f| f.0).to_vec();
    let list_notes = ARRAY_OF_NOTES.map(|f| f.0).to_vec();
    let list_basses = ARRAY_OF_NOTES.map(|f| f.0).to_vec();

    #[allow(unused)]
    let my_selection: select::selection::Selection<'_>;
    #[allow(unused)]
    // let change_data = |data| yew::Callback::from(move |_: yew::Event| alert(data));
    let suz = yew::html! {
            <div class="paneaux">

                <div class="selecteur" >
                    <select class="accords-list" >{ for list_notes.iter().map(|&option|  render_option(option)) }</select>
                    <select class="accords-list">{ for list_options.iter().map(|&option|  render_option(option)) }</select>
                    <hr/>
                    <select class="accords-list" >{ for list_basses.iter().map(|&option|  render_option(option)) }</select>
                </div>
            </div>
    };
    suz
}

fn render_option(option: &str) -> yew::Html {
    let result_option = option;
    yew::html! {
        <option value={result_option.to_string()} >{ option }</option>
    }
}


fn render_octave(option: &str, his_number: u8) -> yew::Html {
    let handle_click = |number: u8| yew::Callback::from(move |_: yew::MouseEvent| piano_touch(number));
    let handle_mouseup = |number: u8| yew::Callback::from(move |_: yew::MouseEvent| piano_up(number));

    let result_option = option;
    let number_option = his_number;
    yew::html! {
        <div>
            <li class={result_option.to_string()} onmousedown={handle_click(number_option)} onmouseup={handle_mouseup(number_option)} ></li>
        </div>
    }
}

fn piano_touch(button_number: u8) {
    let connection_out = create_connection_out();
    // Implémenter votre logique ici en fonction du bouton cliqué
    play_notes(button_number,connection_out);
    // alert(&message);
}

fn piano_up(button_number: u8) {
    let connection_out = create_connection_out();

    // Implémenter votre logique ici en fonction du bouton cliqué
    stop_note(button_number,connection_out);
    // alert(&message);
}

fn play_notes(number: u8,mut connection_out:midir::MidiOutputConnection) {
    // Envoyer un message MIDI pour jouer la note
    send_midi_message(&mut connection_out, 0b10000000, 23+12+number, 64);
    send_midi_message(&mut connection_out, 0x90, 23+12+number, 64);
}

fn stop_note(number: u8,mut connection_out:midir::MidiOutputConnection) {
    // Envoyer un message MIDI pour tuer la note 
    send_midi_message(&mut connection_out, 0b10000000, 23+12+number, 64);
}
fn create_connection_out() -> midir::MidiOutputConnection {
    let result_midi_out = midir::MidiOutput::new("MyMidiOutput");
    let midi_out=result_midi_out.unwrap();
    let binding_port = midi_out.ports();
    let out_port_optional = binding_port.first();
    let retour = match out_port_optional.is_some() {
        true => {
            let out_port=out_port_optional.expect("midi_out ports should be present");
            let out_port_name = midi_out.port_name(out_port).unwrap();
            let connection_out = midi_out.connect(&out_port, &out_port_name).unwrap();
            connection_out
        },
        false=>create_connection_out(),
    };
    retour
}

fn send_midi_message(connection: &mut midir::MidiOutputConnection, status: u8, data1: u8, data2: u8) {
    let message = [status, data1, data2];
    connection.send(&message).expect("Failed to send MIDI message");
}
}
mod outils;
use outils::ui::{AffichePiano, SelecteurAccords};
use yew::*;

fn piano_graphique() {
    Renderer::<SelecteurAccords>::new().render();
    Renderer::<AffichePiano>::new().render();
}
fn main() {
    piano_graphique();
}

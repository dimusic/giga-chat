use yew_router::Routable;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Chat,
    #[at("/settings")]
    Settings,
}

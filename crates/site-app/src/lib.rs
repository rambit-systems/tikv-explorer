#![feature(iter_intersperse)]

use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    <Title text="TiKV Explorer"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=HomePage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn Pair(pair: (values::Value, values::Value)) -> impl IntoView {
  let (key, value) = pair;

  view! {
    <div class="flex flex-row gap-2">
      <Value value=key class="basis-1/3" />
      <Value value=value class="basis-2/3" />
    </div>
  }
}

#[island]
pub fn Value(
  value: values::Value,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let badge_name = match value {
    values::Value::MessagePack(_) => "MsgPack",
    values::Value::Json(_) => "Json",
    values::Value::String(_) => "String",
    values::Value::Bytes(_) => "Bytes",
  };

  let badge_class_color = match value {
    values::Value::MessagePack(_) => "badge-flat-primary",
    values::Value::Json(_) => "badge-flat-secondary",
    values::Value::String(_) => "badge-flat-success",
    values::Value::Bytes(_) => "badge-flat-danger",
  };
  let badge_class = format!("badge flex-none {badge_class_color}");

  let class =
    format!("flex flex-row items-center gap-2 overflow-hidden {class}");

  view! {
    <div class=class>
      <span class=badge_class> { badge_name } </span>
      <span class="flex-none font-mono"> { format!("{:?}", value) } </span>
    </div>
  }
}

#[component]
pub fn PairsTable(pairs: Vec<(values::Value, values::Value)>) -> impl IntoView {
  let rendered_pairs = pairs
    .into_iter()
    .map(|p| view! { <Pair pair=p.clone() /> }.into_view())
    .intersperse(view! { <div class="divider my-2" />}.into_view())
    .collect::<Vec<_>>();

  view! {
    <div class="w-full p-4 border border-gray-6 rounded-lg">
      { rendered_pairs }
    </div>
  }
}

#[island]
pub fn Pairs() -> impl IntoView {
  let pairs = create_resource(|| (), |_| get_pairs());
  let consume_pairs_resource = move || match pairs.get() {
    Some(Ok(pairs)) => view! { <PairsTable pairs=pairs /> }.into_view(),
    Some(Err(e)) => {
      view! { <div> { format!("Error: {:?}", e) } </div> }.into_view()
    }
    None => view! { <div> { "Loading..." } </div> }.into_view(),
  };

  view! {
    <Suspense fallback=|| view! {"Loading..."}>
      { consume_pairs_resource }
    </Suspense>
  }
}

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="container mx-auto">
      <Pairs />
    </div>
  }
}

#[component]
pub fn NavBar() -> impl IntoView {
  view! {
    <nav class="flex items-center justify-between flex-wrap bg-teal-500 p-6">
      <div class="flex items-center flex-shrink-0 text-white mr-6">
        <span class="font-semibold text-xl tracking-tight">Leptos</span>
      </div>
    </nav>
  }
}

#[server]
pub async fn get_pairs(
) -> Result<Vec<(values::Value, values::Value)>, ServerFnError> {
  let client = probe::Client::new(vec!["127.0.0.1:2379".to_string()])
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

  let pairs = client
    .get_all()
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

  Ok(pairs)
}

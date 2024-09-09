#![feature(iter_intersperse)]

use leptos::{
  either::{Either, EitherOf3},
  prelude::*,
  spawn::spawn_local,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options/>
        <MetaTags/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    <Title text="TiKV Explorer"/>

    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
        </Routes>
      </main>
    </Router>
  }
}

#[component]
pub fn Pair(pair: (values::Value, values::Value)) -> impl IntoView {
  let (key, value) = pair;

  view! {
    <div class="flex flex-row gap-2">
      <Value value=key class="basis-1/3" />
      <div class="divider divider-vertical mx-0 h-6" />
      <Value value=value class="basis-2/3" />
    </div>
  }
}

#[island]
pub fn Value(
  value: values::Value,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let (long, set_long) = signal(false);

  let display = Memo::new({
    let value = value.clone();
    move |_| match long() {
      false => value.pretty(),
      true => value.pretty_long(),
    }
  });

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

  let container_class =
    format!("flex flex-row items-center gap-2 overflow-hidden {class}");
  let badge_class = format!("badge flex-none {badge_class_color}");
  let display_class = move || {
    format!(
      "flex-auto font-mono truncate {}",
      if long() { "whitespace-pre-wrap" } else { "" }
    )
  };
  let expand_icon_class = move || {
    format!(
      "size-5 text-content2 transition-transform ease-in-out \
       transition-height duration-300 {}",
      if long() { "rotate-180" } else { "" }
    )
  };

  let toggle_long = move |_| set_long(!long());

  view! {
    <div class=container_class>
      <span class=badge_class> { badge_name } </span>
      <span class=display_class> { display } </span>
      <button class="flex-none" on:click=toggle_long>
        <HeroIconsChevronDown {..} class=expand_icon_class />
      </button>
      { move || view! { <CopyButton text=display() /> } }
    </div>
  }
}

#[island]
pub fn CopyButton(#[prop(optional, into)] text: String) -> impl IntoView {
  let click_action = {
    let text = text.clone();
    move |_| {
      let clipboard = web_sys::window().unwrap().navigator().clipboard();
      let promise = clipboard.write_text(&text);
      spawn_local(async move {
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
      });
    }
  };

  view! {
    <button class="flex-none" on:click=click_action>
      <HeroIconsClipboardDocument {..} class="size-5 text-content2" />
    </button>
  }
}

#[component]
pub fn PairsTable(pairs: Vec<(values::Value, values::Value)>) -> impl IntoView {
  let rendered_pairs = pairs
    .into_iter()
    .map(|p| Either::Left(view! { <Pair pair=p.clone() /> }))
    .intersperse_with(|| Either::Right(view! { <div class="divider my-2" />}))
    .collect::<Vec<_>>();

  let inner_table = match rendered_pairs.is_empty() {
    true => {
      Either::Left(view! { <div> { "No key/value pairs found." } </div> })
    }
    false => Either::Right(rendered_pairs),
  };

  view! {
    <div class="w-full p-4 border border-gray-6 rounded-lg">
      { inner_table }
    </div>
  }
}

#[component]
pub fn Pairs() -> impl IntoView {
  let pairs = Resource::new(|| (), |_| get_pairs());
  let consume_pairs_resource = move || match pairs.get() {
    Some(Ok(pairs)) => EitherOf3::A(view! { <PairsTable pairs=pairs /> }),
    Some(Err(e)) => {
      EitherOf3::B(view! { <div> { format!("Error: {:?}", e) } </div> })
    }
    None => EitherOf3::C(view! { <div> { "Loading..." } </div> }),
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
      <div class="mb-8" />
      <Pairs />
    </div>
  }
}

#[server]
pub async fn get_pairs(
) -> Result<Vec<(values::Value, values::Value)>, ServerFnError> {
  let client = probe::Client::new(vec!["127.0.0.1:2379".to_string()])
    .await
    .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

  let pairs = client
    .get_all()
    .await
    .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

  Ok(pairs)
}

#[component]
pub fn HeroIconsClipboardDocument() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z" />
    </svg>
  }
}

#[component]
pub fn HeroIconsChevronDown() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
    </svg>
  }
}

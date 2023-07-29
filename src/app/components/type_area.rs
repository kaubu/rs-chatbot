use leptos::{*, html::Input};

const TYPE_AREA_DIV_CLASS: &str = "h-24 w-full fixed bottom-0 flex\
justify-center items-center p-5 bg-white border-t border-gray-300";
const FORM_CLASS: &str = "w-full flex justify-center items-center gap-4";
const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border border-gray-300 rounded-full";
const SUBMIT_CLASS: &str = "h-full p-4 bg-blue-500 text-white rounded-full\
cursor-pointer";

#[component]
pub fn TypeArea(
    cx: Scope,
    send: Action<String, Result<String, ServerFnError>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    
    view! {cx,
        <div class={TYPE_AREA_DIV_CLASS}>
            <form
            class={FORM_CLASS}
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input doesn't exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input type="text" class={TEXT_AREA_CLASS} node_ref=input_ref/>
                <input type="submit" class={SUBMIT_CLASS}/>
            </form>
        </div>
    }
}
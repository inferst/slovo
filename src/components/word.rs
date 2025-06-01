use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct WordProps {
    pub word: String,
    pub user: String,
    pub color: String,
    pub distance: i32,
    pub animate: bool,
}

fn get_inverse_log_progress(value: f64, max: f64) -> f64 {
    let min = 1.0;
    let clamped_value = value.max(min);
    let log_min = min.ln();
    let log_max = max.ln();
    let log_value = clamped_value.ln();

    let normalized = (log_value - log_min) / (log_max - log_min);
    let percent = (1.0 - normalized) * 100.0;

    percent.clamp(0.0, 100.0)
}

#[component]
pub fn Word(props: WordProps) -> Element {
    let width = get_inverse_log_progress(props.distance as f64, 4000.);
    let color = match props.distance {
        1 => "from-purple-700 to-purple-500",
        2..=100 => "from-green-700 to-green-500",
        101..=1000 => "from-yellow-700 to-yellow-500",
        1001..=3000 => "from-orange-700 to-orange-500",
        _ => "from-red-700 to-red-500",
    };

    rsx! {
        div {
            class: "relative",

            div {
                class: "relative mb-2 rounded-md font-bold border-2",
                class: if props.animate { "border-white" },

                div {
                    class: "absolute left-0 top-0 h-[100%] w-[var(--width)] p-1 z-0 bg-linear-to-r {color}",
                    style: "--width: {width}%",
                }

                div {
                    class: "flex px-4 py-2 bg-gray-800 items-center text-xl",

                    div {
                        class: "relative grow z-10",
                        "{props.word}"
                    }

                    div {
                        class: "relative z-10",
                        "{props.distance}"
                    }
                }
            }

            div {
                class: "text-[var(--color)] absolute left-[100%] top-0 h-[100%] mx-4 py-2 whitespace-nowrap",
                style: "--color: {props.color}",
                "{props.user}"
            }
        }
    }
}

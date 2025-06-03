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
    let smoothed = normalized.powf(3.0);
    let percent = (1.0 - smoothed) * 100.0;

    percent.clamp(0.0, 100.0)
}

#[component]
pub fn Word(props: WordProps) -> Element {
    let max = 3000;
    let width = get_inverse_log_progress(props.distance as f64, max as f64);
    let color = match width {
        100. => "from-blue-500 to-purple-500",
        75.0..=100.0 => "from-green-700 to-green-500",
        50.0..=75.0 => "from-yellow-700 to-yellow-500",
        25.0..=50.0 => "from-orange-700 to-orange-500",
        _ => "from-red-700 to-red-500",
    };

    rsx! {
        div {
            class: "relative font-bold",

            div {
                class: "relative mb-2 rounded-md",
                class: if props.animate { "border-2 border-white" },

                div {
                    class: "absolute rounded-md left-0 top-0 h-[100%] w-[var(--width)] p-1 z-0 bg-linear-to-r {color}",
                    style: "--width: {width}%",
                }

                div {
                    class: "rounded-md flex px-4 py-2 bg-gray-800 items-center text-shadow-md",

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
                class: "text-[var(--color)] absolute left-[100%] top-0 h-[100%] mx-4 py-2 whitespace-nowrap flex items-center",
                style: "--color: {props.color}",
                "{props.user}"
            }
        }
    }
}

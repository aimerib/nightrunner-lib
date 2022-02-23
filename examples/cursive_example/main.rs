use cursive::align::{HAlign, VAlign};
use cursive::event::EventResult;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Style};
use cursive::utils::span::SpannedString;
use cursive::view::{Nameable, Resizable, ScrollStrategy};
use cursive::views::{
    Dialog, DummyView, EditView, Layer, LinearLayout, NamedView, PaddedView, ResizedView,
    ScrollView, TextContent, TextView,
};
use cursive::{Cursive, View};
use cursive_aligned_view::Alignable;
use nightrunner_lib::parser::interpreter::{EventMessage, MessageParts};
use nightrunner_lib::{NightRunner, NightRunnerBuilder, ParsingResult};

static THEME_COLORS: [(&str, Color); 10] = [
    ("background", Color::Rgb(17, 17, 17)),
    ("shadow", Color::Dark(BaseColor::Green)),
    ("view", Color::Rgb(17, 17, 17)),
    ("primary", Color::Dark(BaseColor::Green)),
    ("secondary", Color::Dark(BaseColor::Green)),
    ("tertiary", Color::Dark(BaseColor::Green)),
    ("title_primary", Color::Rgb(255, 0, 127)),
    ("title_secondary", Color::Dark(BaseColor::Green)),
    ("highlight", Color::Dark(BaseColor::Green)),
    ("highlight_text", Color::Dark(BaseColor::Black)),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nr = NightRunnerBuilder::new()
        .with_path_for_config("./fixtures/")
        .build();
    let intro_text = nr.game_intro();
    let first_room_text = nr.first_room_text()?.message;
    let mut siv = cursive::default();
    let mut theme = siv.current_theme().clone();
    theme.borders = BorderStyle::Simple;
    theme.shadow = false;
    for tc in THEME_COLORS.iter() {
        theme.palette.set_color(tc.0, tc.1)
    }
    siv.set_theme(theme);
    siv.set_user_data(nr);

    let main_view = Dialog::around(PaddedView::lrtb(
        0,
        0,
        1,
        0,
        ScrollView::new(
            LinearLayout::vertical().child(TextView::new(first_room_text).with_name("room_text")),
        )
        .scroll_strategy(ScrollStrategy::StickToBottom)
        .with_name("scroll_area"),
    ))
    .title("Cursive Example")
    .title_position(HAlign::Center)
    .with_name("main_content")
    .fixed_width(90)
    .fixed_height(25);

    let input_view = Dialog::around(
        LinearLayout::horizontal().child(TextView::new("> ")).child(
            EditView::new()
                .filler(" ")
                .on_submit(on_submit)
                .style(ColorStyle::new(
                    Color::Rgb(17, 17, 17),
                    Color::Light(BaseColor::Green),
                ))
                .with_name("input")
                .full_width(),
        ),
    )
    .fixed_width(90);
    let view = ResizedView::with_full_screen(
        LinearLayout::vertical()
            .child(main_view)
            .child(input_view)
            .fixed_height(28)
            .align_center(),
    );
    siv.add_fullscreen_layer(PaddedView::lrtb(2, 2, 2, 1, view));
    siv.add_fullscreen_layer(PaddedView::lrtb(2, 2, 2, 1, intro(intro_text)));

    siv.run();
    Ok(())
}

fn on_submit(siv: &mut Cursive, query: &str) {
    let nr = siv.user_data::<NightRunner>().unwrap();

    let result = nr.parse_input(query);
    match result {
        Ok(parsing_result) => match parsing_result {
            ParsingResult::NewItem(item_message) => siv
                .call_on_name("room_text", |view: &mut TextView| view.append(item_message))
                .unwrap(),
            ParsingResult::DropItem(drop_message) => siv
                .call_on_name("room_text", |view: &mut TextView| view.append(drop_message))
                .unwrap(),
            ParsingResult::Look(text) => siv.add_layer(
                Dialog::around(TextView::new(text))
                    .dismiss_button("OK")
                    .h_align(HAlign::Center)
                    .align_center(),
            ),
            ParsingResult::Inventory(inventory) => siv.add_layer(
                Dialog::around(TextView::new(inventory))
                    .dismiss_button("OK")
                    .h_align(HAlign::Center)
                    .align_center(),
            ),
            ParsingResult::EventSuccess(event_message) => {
                let EventMessage {
                    message,
                    message_parts,
                    templated_words: _,
                } = event_message;
                siv.call_on_name("room_text", |view: &mut TextView| {
                    view.set_content(message);
                    view.append(&message_parts.get(&MessageParts::EventText).unwrap().clone());
                })
                .unwrap();
            }
            ParsingResult::SubjectNoEvent(subject_text) => siv
                .call_on_name("room_text", |view: &mut TextView| {
                    view.append("\n".to_owned() + &subject_text);
                })
                .unwrap(),
            ParsingResult::Help(help_text) => {
                siv.add_fullscreen_layer(ResizedView::with_full_screen(
                    Layer::new(
                        Dialog::around(TextView::new(help_text))
                            .dismiss_button("OK")
                            .h_align(HAlign::Center)
                            .fixed_height(28)
                            .fixed_width(90),
                    )
                    .align_center(),
                ))
            }
            ParsingResult::Quit => siv.quit(),
        },
        Err(e) => siv.add_layer(
            Dialog::around(TextView::new(format!("{}", e)))
                .dismiss_button("OK")
                .h_align(HAlign::Center)
                .align_center(),
        ),
    };
    siv.call_on_name("input", |view: &mut EditView| view.set_content(""));
    siv.call_on_name(
        "scroll_area",
        |view: &mut ScrollView<NamedView<TextView>>| {
            if !view.is_at_bottom() {
                view.set_scroll_strategy(ScrollStrategy::StickToBottom)
            } else {
                EventResult::Ignored
            }
        },
    );
}

pub fn intro(intro_text: String) -> impl View {
    let title: TextContent = TextContent::new(intro_text);

    let mut styled_body: SpannedString<Style> = SpannedString::new();
    styled_body.append("Press Enter to begin your journey.\nType ");
    styled_body.append_styled("quit", Style::from(Color::Rgb(255, 0, 127)));
    styled_body.append(" to exit and ");
    styled_body.append_styled("help", Style::from(Color::Rgb(255, 0, 127)));
    styled_body.append(" for instructions");
    let body: TextContent = TextContent::new(styled_body);

    ResizedView::with_full_screen(
        Layer::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(PaddedView::lrtb(
                        0,
                        0,
                        1,
                        0,
                        TextView::new_with_content(title.clone()).h_align(HAlign::Center),
                    ))
                    .child(DummyView.fixed_height(2))
                    .child(ResizedView::with_full_height(
                        TextView::new_with_content(body.clone())
                            .v_align(VAlign::Center)
                            .h_align(HAlign::Center),
                    )),
            )
            .dismiss_button("Start")
            .h_align(HAlign::Center)
            .fixed_height(28)
            .fixed_width(90),
        )
        .align_center(),
    )
}

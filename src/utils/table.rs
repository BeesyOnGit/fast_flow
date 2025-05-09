use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Border, Color, Padding, Panel, Style, Width,
        object::{Columns, Object, Rows, Segment},
        style::{BorderColor, HorizontalLine},
    },
};
use terminal_size::{Width as TermWidth, terminal_size};

use super::structs::WatchStats;

pub fn create_table<T>(data: &Vec<T>, title: &'static str) -> Table
where
    T: Tabled,
{
    let t_width = terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(100);

    // let term_width = (f32::from(t_width) * size) as usize;

    let mut custom_table = Table::new(data);
    custom_table
        // Add header panel with centered text
        .with(Panel::header(title))
        .with(Style::modern_rounded())
        .modify(Columns::first(), Alignment::left())
        .modify(Columns::first(), Padding::new(0, 0, 0, 0))
        .modify(Rows::first(), Alignment::center())
        .modify(Rows::first(), Color::FG_BRIGHT_GREEN)
        .modify(Rows::single(1), Color::FG_CYAN)
        .modify(Rows::first(), Padding::new(0, 0, 0, 0))
        .modify(
            Rows::single(0).not(Columns::first()).not(Columns::last()),
            Border::inherit(Style::blank()).left(' ').right(' '),
        )
        .with(
            Style::modern_rounded().horizontals([
                (
                    1,
                    HorizontalLine::inherit(Style::modern_rounded()).intersection('┬'), // <─ keeps type On, just new char
                ),
                (
                    0,
                    HorizontalLine::inherit(Style::modern_rounded())
                        .intersection('─')
                        .left('╭') // keep rounded corners
                        .right('╮'), // <─ keeps type On, just new char
                ),
            ]),
        )
        .with(Width::wrap(t_width)) // don’t exceed
        // .with(Width::wrap(term_width)) // don’t exceed
        // .with(Width::increase(term_width))
        .modify(
            Segment::all(), // every cell
            BorderColor::filled(Color::rgb_fg(127, 127, 127)),
        );
    // .modify(
    //     Rows::first(),
    //     BorderColor::filled(Color::rgb_fg(127, 127, 127)),
    // )
    // .modify(
    //     Columns::first(),
    //     BorderColor::filled(Color::rgb_fg(127, 127, 127)),
    // );

    return custom_table;
}

pub fn watch_status_table(data: Vec<WatchStats>, title: &'static str) -> Table {
    let mut table = create_table::<WatchStats>(&data, title);

    for (idx, dat) in data.iter().enumerate() {
        let mut color: Color = Color::FG_WHITE;

        if &dat.status == "watched" {
            color = Color::FG_BRIGHT_GREEN;
        };

        table.modify(
            // Columns::last().and(Rows::single(idx + 2)),
            Rows::single(idx + 2).intersect(Columns::single(6)), // that one cell
            color,
        );
    }

    return table;
}

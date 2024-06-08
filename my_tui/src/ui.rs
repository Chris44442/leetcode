use ratatui::{
    prelude::*,
    // layout::{Constraint, Direction, Layout, Rect},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    // widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    widgets::{block::*,*,Block, Borders, List, ListItem, Paragraph, block::Position},
    Frame, prelude::Alignment,
};

// use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(f.size());

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(chunks[1]);
    let super_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(62),
            Constraint::Percentage(38),
        ])
        .split(footer_chunks[0]);

    // f.render_widget(title_block.clone(), chunks[1]);
    // let mut list_items = Vec::<ListItem>::new();
    // let list_items = Vec::<ListItem>::new();

    // for key in app.pairs.keys() {
    //     list_items.push(ListItem::new(Line::from(Span::styled(
    //         format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
    //         Style::default().fg(Color::Yellow),
    //     ))));
    // }

    // let list = List::new(list_items);

    // f.render_widget(list, chunks[1]);
    // let current_navigation_text = vec![
        // The first half of the text
        // match app.current_screen {
        //     CurrentScreen::Main => {
        //         Span::styled("Normal Mode", Style::default().fg(Color::Green))
        //     }
            // CurrentScreen::Editing => {
            //     Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            // }
            // CurrentScreen::Exiting => {
            //     Span::styled("Exiting", Style::default().fg(Color::LightRed))
            // }
        // }
        // .to_owned(),
        // A white divider bar to separate the two sections
        // Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        // {
            // if let Some(editing) = &app.currently_editing {
            //     match editing {
            //         CurrentlyEditing::Key => Span::styled(
            //             "Editing Json Key",
            //             Style::default().fg(Color::Green),
            //         ),
            //         CurrentlyEditing::Value => Span::styled(
            //             "Editing Json Value",
            //             Style::default().fg(Color::LightGreen),
            //         ),
            //     }
            // } else {
                // Span::styled(
                //     "Not Editing Anything",
                //     Style::default().fg(Color::DarkGray),
                // )
            // }
    //     },
    // ];

    // let mode_footer = Paragraph::new(Line::from(current_navigation_text))
    //     .block(Block::default().borders(Borders::ALL));

    // let current_keys_hint = {
    //     match app.current_screen {
    //         CurrentScreen::Main => Span::styled(
    //             "(q) to quit / (e) to make new pair",
    //             Style::default().fg(Color::Red),
    //         ),
            // CurrentScreen::Editing => Span::styled(
            //     "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
            //     Style::default().fg(Color::Red),
            // ),
            // CurrentScreen::Exiting => Span::styled(
            //     "(q) to quit / (e) to make new pair",
            //     Style::default().fg(Color::Red),
            // ),
    //     }
    // };

    // let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
    //     .block(Block::default().borders(Borders::ALL));
    let dark_brownish_red = Color::Rgb(235,48,44);
    let greensish_blue = Color::Rgb(122, 170, 255);
    let darkish_violet = Color::Rgb(33, 32, 40);
    let black = Color::Rgb(18,18,18);
    let not_quite_black = Color::Rgb(48,48,48);
    let lighter_green = Color::Rgb(82,106,86);
    let lighter_purplish = Color::Rgb(83,79,127);
    let light_pinkish = Color::Rgb(126,81,81);
    let yellow_green = Color::Rgb(97,97,67);
    let almost_white = Color::Rgb(241, 238, 235);
    // let reddish = Color::Rgb(199, 105, 79);
    let reddish = Color::Rgb(255, 135, 15);

    let user_block = Block::default()
        .title("Status")
        .title_style(Style::default().bg(black).fg(Color::White))
        .borders(Borders::ALL)
        .border_style(Style::default().bg(black).fg(lighter_green))
        .style(Style::default().bg(black).fg(Color::White));
        
    let user_input_paragraph = Paragraph::new(Text::styled(
        "",
        Style::default().fg(Color::White),
    ))
    .block(user_block.clone());
    f.render_widget(user_input_paragraph.clone(), chunks[0]);

    let isle2_span = Span::styled("s", Style::default().fg(dark_brownish_red));
    let isle2_title = vec![Span::raw(" i"), isle2_span, Span::raw("le2 ")];
    let isle1_span = Span::styled(" i", Style::default().fg(dark_brownish_red));
    let isle1_title = vec![isle1_span, Span::raw("sle1 ")];

    let title_block = Block::default()
        .title(Title::from(isle1_title).alignment(Alignment::Left))
        .title_style(Style::default().bg(black).fg(Color::White))
        .title(Title::from(isle2_title).alignment(Alignment::Left))
        .title_style(Style::default().bg(black).fg(Color::White))
        .borders(Borders::ALL)
        .border_style(Style::default().bg(black).fg(light_pinkish))
        .style(Style::default().bg(black));

    let style1 = Style::default().bg(not_quite_black).fg(almost_white);
    let style2 = Style::default().fg(almost_white);
    let span0 = Span::styled(   "Address:   Value:     Access:  Description:", style2);
    let span1 = Span::styled("0000_0000  6d65_203D  RW       Test register", style1);
    let span2 = Span::styled("0000_0004  7665_7273   R       9..8 Azimuth Emulation, 4 ISLE1 LED Blinking, 0 ISLE1 Reset", style2);
    let span3 = Span::styled("0000_0008  220A_6564  RW       Interrupt registers", style1);
    let span4 = Span::styled("0000_000C  696F_6e20  RW       PED Status: 10 Timeout, 9 Communication Err, 8 Ready; Azimuth Encoder: 3 Timeout, 2 Packet Timing Err, 1 Communication Err, 0 Parity Err", style2);
    let span5 = Span::styled("0000_0010  6AC1_C802  RW       0 MK_PWR_ON", style1);

    let texxt = Text::from(vec![
        Line::from(vec![span0]),
        Line::from(vec![span1]),
        Line::from(vec![span2]),
        Line::from(vec![span3]),
        Line::from(vec![span4]),
        Line::from(vec![span5]),
    ]);

    let spezialparagraph = Paragraph::new(texxt)
        .block(title_block.clone())
        .scroll((0,0));
    f.render_widget(spezialparagraph, footer_chunks[1]);

    let rbf_span = Span::styled("c", Style::default().fg(dark_brownish_red));
    let rbf_title = vec![rbf_span, Span::raw("onfig")];
    let title_block = Block::default()
        .title("rbf")
        .title_style(Style::default().bg(black).fg(Color::White))
        .title(Title::from(rbf_title).alignment(Alignment::Right))
        // .title_style(Style::default().fg(Color::Green))
        // .title_style(Style::default().fg(Color::Rgb(190, 95, 190)))
        .borders(Borders::ALL)
        .border_style(Style::default().bg(black).fg(lighter_purplish))
        .style(Style::default().bg(black));
        // .style(Style::default().bg(Color::Rgb(28, 25, 38)));

    let title = Paragraph::new(Text::styled(
        "   path: /root/sdcard/fpga.rbf
version: v0.05.21
   date: 2023-12-22",
        // Style::default().fg(Color::DarkGray),
        Style::default().fg(almost_white),
        // Style::default().fg(not_quite_black),
    ))
    .block(title_block.clone());
    f.render_widget(title, super_chunks[1]);

    let title_block = Block::default()
        .title("Misc")
        .title_style(Style::default().bg(black).fg(Color::White))
        .borders(Borders::ALL)
        .border_style(Style::default().bg(black).fg(yellow_green))
        .style(Style::default().bg(black));
    let span6 = vec![Span::raw("FPGA Accesses: 65")];
    let span7 = vec![Span::styled("FAN0: ",Style::default().fg(almost_white)), Span::styled("85", Style::default().fg(reddish)), Span::styled("%",Style::default().fg(almost_white))];
    let span8 = vec![Span::styled("FAN1: ",Style::default().fg(almost_white)), Span::styled("84", Style::default().fg(reddish)), Span::styled("%",Style::default().fg(almost_white))];
    let span9 = vec![Span::styled("FAN2: ",Style::default().fg(almost_white)), Span::styled("85", Style::default().fg(reddish)), Span::styled("%",Style::default().fg(almost_white))];
    let span10 = vec![Span::styled("FAN3: ",Style::default().fg(almost_white)), Span::styled("87", Style::default().fg(reddish)), Span::styled("%",Style::default().fg(almost_white))];
    let texxxt = Text::from(vec![
        Line::from(span6),
        Line::from(span7),
        Line::from(span8),
        Line::from(span9),
        Line::from(span10),
    ]);
    let title = Paragraph::new(texxxt)
    .block(title_block.clone());
    f.render_widget(title, super_chunks[0]);
    // f.render_widget(key_notes_footer, footer_chunks[1]);

    // if let Some(editing) = &app.currently_editing {
    //     let popup_block = Block::default()
    //         .title("Enter a new key-value pair")
    //         .borders(Borders::NONE)
    //         .style(Style::default().bg(Color::DarkGray));

    //     let area = centered_rect(60, 25, f.size());
    //     f.render_widget(popup_block, area);

    //     let popup_chunks = Layout::default()
    //         .direction(Direction::Horizontal)
    //         .margin(1)
    //         .constraints([
    //             Constraint::Percentage(50),
    //             Constraint::Percentage(50),
    //         ])
    //         .split(area);

    //     let mut key_block = Block::default().title("Key").borders(Borders::ALL);
    //     let mut value_block =
    //         Block::default().title("Value").borders(Borders::ALL);

    //     let active_style =
    //         Style::default().bg(Color::LightYellow).fg(Color::Black);

    //     match editing {
    //         CurrentlyEditing::Key => key_block = key_block.style(active_style),
    //         CurrentlyEditing::Value => {
    //             value_block = value_block.style(active_style)
    //         }
    //     };

    //     let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
    //     f.render_widget(key_text, popup_chunks[0]);

    //     let value_text =
    //         Paragraph::new(app.value_input.clone()).block(value_block);
    //     f.render_widget(value_text, popup_chunks[1]);
    // }

    // if let CurrentScreen::Exiting = app.current_screen {
    //     f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
    //     let popup_block = Block::default()
    //         .title("Y/N")
    //         .borders(Borders::NONE)
    //         .style(Style::default().bg(Color::DarkGray));

    //     let exit_text = Text::styled(
    //         "Would you like to output the buffer as json? (y/n)",
    //         Style::default().fg(Color::Red),
    //     );
    //     // the `trim: false` will stop the text from being cut off when over the edge of the block
    //     let exit_paragraph = Paragraph::new(exit_text)
    //         .block(popup_block)
    //         .wrap(Wrap { trim: false });

    //     let area = centered_rect(60, 25, f.size());
    //     f.render_widget(exit_paragraph, area);
    // }
}

// helper function to create a centered rect using up certain percentage of the available rect `r`
// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     // Cut the given rectangle into three vertical pieces
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     // Then cut the middle vertical piece into three width-wise pieces
//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1] // Return the middle chunk
// }


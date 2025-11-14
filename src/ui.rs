/// UI utilities for elegant text dividers and styling with multi-color ASCII art

use colored::*;
use rand::prelude::*;
use std::collections::HashMap;

/// ASCII art categories from the gist
pub struct AsciiArtCategories;

impl AsciiArtCategories {
    /// Cute Sparkles category
    #[allow(dead_code)]
    pub fn cute_sparkles() -> Vec<&'static str> {
        vec![
            "✧･ﾟ: *✧･ﾟ:* 　　 *:･ﾟ✧*:･ﾟ✧",
            ".・゜゜・　　・゜゜・．",
            "｡･ﾟﾟ･　　･ﾟﾟ･｡",
            "༶•┈┈⛧┈♛ ♛┈⛧┈┈•༶",
            "✧༺♥༻∞　　∞༺♥༻✧",
            "*＊✿❀　❀✿＊*",
            "⋆ ˚｡⋆୨୧˚　˚୨୧⋆｡˚ ⋆",
            "｡o°✥✤✣ 　　 ✣✤✥°o｡",
            "♬♩♪♩　　♩♪♩♬",
            "*:..｡o○　　○o｡..:*",
            "ﾟ+*:ꔫ:*﹤　　﹥*:ꔫ:*+ﾟ",
            "*+:｡.｡　　｡.｡:+*",
            "♥*♡∞:｡.｡　　｡.｡:∞♡*♥",
            "‧̍̊˙˚˙ᵕ꒳ᵕ˙˚˙　　˙˚˙ᵕ꒳ᵕ˙˚˙‧̍̊",
            "*°:⋆ₓₒ　　ₓₒ⋆:°*",
            "ₓ˚. ୭ ˚○◦˚.˚◦○˚ ୧ .˚ₓ",
            "ˑ༄ؘ ۪۪۫۫ ▹▫◃ ۪۪۫۫ ༄ؘ ˑ",
            "° 𐐪𐑂 ♡ 𐐪𐑂 ₒ 𐐪𐑂 ♡ 𐐪𐑂 °",
            "∞ ₒ ˚ ° 𐐒𐐚 ° ˚ ₒ ∞",
            "｡ₓ ू ₒ ु ˚ ू ₒ ु ₓ｡",
            "☆♬○♩●♪✧♩　　♩✧♪●♩○♬☆",
            "⭑･ﾟﾟ･*:༅｡.｡༅:*ﾟ:*:✼✿　　✿✼:*ﾟ:༅｡.｡༅:*･ﾟﾟ･⭑",
        ]
    }

    /// Galactic Sparkles category
    #[allow(dead_code)]
    pub fn galactic_sparkles() -> Vec<&'static str> {
        vec![
            "｡･:*:･ﾟ★,｡･:*:･ﾟ☆　　 ｡･:*:･ﾟ★,｡･:*:･ﾟ☆",
            ".・゜-: ✧ :-　　-: ✧ :-゜・．",
            "⋇⋆✦⋆⋇　 ⋇⋆✦⋆⋇",
            "⭒❃.✮:▹　　◃:✮.❃⭒",
            "‧͙⁺˚･༓☾　　☽༓･˚⁺‧͙",
            ".｡ﾟ+..｡　　　ﾟ+..｡ﾟ+",
            "☆.｡.:　　.｡.:☆",
            "☆○o。　　。o○☆",
            ".⋆｡⋆☂˚｡⋆｡˚☽˚｡⋆.",
            "⤹⋆⸙͎۪۫｡˚۰˚☽˚⁀➷｡˚⸙͎۪۫⋆ ༄",
            "ع˖⁺ ☁⋆ ୭ 🕊.⋆｡⋆༶⋆˙⊹",
            "ˏ 𓏧 𓏲 𓏲 𓏲 𓋒 𓏲 𓏲 𓏲 𓏲 𓏧 ˎ",
            "‧̍̊˙· 𓆝.° ｡˚𓆛˚｡ °.𓆞 ·˙‧̍̊",
            "｡ﾟ❁ུ۪ °ₒ 𓂂 ˚ 𓂂 ₒ ° ₒ 𓂂 ˚˖⋆",
            "⋆┈┈｡ﾟ❃ུ۪ ❀ུ۪ ❁ུ۪ ❃ུ۪ ❀ུ۪ ﾟ｡┈┈⋆",
            ".｡❅⋆⍋∞｡∞⍋⋆❅｡.",
            "▸ 🎕 ┈┈┈┈ 🎕 ┈┈┈┈ 🎕 ◂",
            "᠃ ⚘᠂ ⚘ ˚ ⚘ ᠂ ⚘ ᠃",
            "・‥…━━━━━━━☆☆━━━━━━━…‥・",
            "｡☆✼★━━━━━━━━━━━━★✼☆｡",
            "★・・・・・・★・・・・・・★・・・・・・★",
            "【☆】★【☆】★【☆】★【☆】★【☆】",
            "»»——⍟——««",
            ".・。.・゜✭・.・✫・゜・。.",
        ]
    }

    /// Nature mood category
    #[allow(dead_code)]
    pub fn nature_mood() -> Vec<&'static str> {
        vec![
            ".⋆｡⋆☂˚｡⋆｡˚☽˚｡⋆.",
            "⤹⋆⸙͎۪۫｡˚۰˚☽˚⁀➷｡˚⸙͎۪۫⋆ ༄",
            "ع˖⁺ ☁⋆ ୭ 🕊.⋆｡⋆༶⋆˙⊹",
            "ˏ 𓏧 𓏲 𓏲 𓏲 𓋒 𓏲 𓏲 𓏲 𓏲 𓏧 ˎ",
            "‧̍̊˙· 𓆝.° ｡˚𓆛˚｡ °.𓆞 ·˙‧̍̊",
            "｡ﾟ❁ུ۪ °ₒ 𓂂 ˚ 𓂂 ₒ ° ₒ 𓂂 ˚˖⋆",
            "⋆┈┈｡ﾟ❃ུ۪ ❀ུ۪ ❁ུ۪ ❃ུ۪ ❀ུ۪ ﾟ｡┈┈⋆",
            ".｡❅⋆⍋∞｡∞⍋⋆❅｡.",
            "▸ 🎕 ┈┈┈┈ 🎕 ┈┈┈┈ 🎕 ◂",
            "᠃ ⚘᠂ ⚘ ˚ ⚘ ᠂ ⚘ ᠃",
        ]
    }

    /// Non-language symbols category
    #[allow(dead_code)]
    pub fn non_language_symbols() -> Vec<&'static str> {
        vec![
            "❒  ⭒  ⋆   ⌯ ᴗ -  𖥻  %",
            "〄  ▨  ▦  ▩  ◌  ◍  ◔  ◕  ❀",
            " ✿  ❁  ✾  ❖  ☆  ★",
            "□  ■  ✮  ✰  ？ ！ ⍝  ∗  ✦  ✧",
            " ⎙  ‹𝟹  ‹𝟥  ♡  ♥︎",
            "☺︎  ⊹  ☕︎   ࣪ ˖",
            " › ⌗ ▹ ⚠︎",
            "⠀⑅⠀⠀✦⠀",
            "⠀⋆⠀⬪⠀⠀╰╮⠀♡⠀",
            "⬪˙     ⠀⌕⠀⊹ ⠀⠀",
            "﹟⠀‹𝟹⠀+⠀ ..",
            "∞⠀⠀⌗⠀★⠀♥︎⠀⭒⠀ʚ ♡⠀",
            "﹫⠀⠀﹠⠀⠀◪⠀！⋆",
            "  ▦  ◐⠀≀⠀",
            "﹗ 💭",
            "• ˖˓ ★  ˖▸◂ ⋆  ꜝꜞ◗  ˖ ࣪ ‹",
            "• • • • •",
            "⭓   ⭔ ∞ ■",
            "┄┄┄",
            "｡ﾟﾟ･｡･ﾟﾟ｡",
            "ﾟ。",
            "　ﾟ･｡･ﾟ",
            "◠ ◡ ⌦ ⌫ ⌞ ⌝",
            ",, ✰★  ･｡",
            "⇡ ╴",
            " ↳ ↰ ⁺◟ ➢",
            " ‿︵ ❛ ⊰ ⋱⋰",
            "︿ ，；↻  .·˙·. ⇾ ﹏",
            "⩩ ◜◝ ◟◞",
            "❍ ❏ ⁞ ﹫︶ ⟆",
            "⿻⌇",
            "/) /)",
            "  ╰  ╯ ╮ ╭",
            "⁾⁾ ‧₊",
            " ⑅ ␥ 。",
            "×  ᵎᵎ     ⿻  ・．⭔ ︰",
            "✦  ꜜ  ！˚₊ ‹3﹒",
            "⧜ ˃ ︿ ˂   ︵ ⊹",
            "・✿   ♡ ❀",
            "˶ ᵔᵔ ᵕ  ᴗ  /) /) ・",
            "🎔 🎕 🗦 🗧",
            "︰ ⨝    ⨞    ⨟ ⨯",
        ]
    }

    /// Stylized box patterns from the gist - organized as (top, left_char, right_char, bottom)
    pub fn boxes() -> Vec<(&'static str, char, char, &'static str)> {
        vec![
            ("┎╌╌╯╰╌╌┒", '┃', '┃', "┖╌╌╮╭╌╌┚"),
            ("╭────╮", '│', '│', "╰────╯"),
            ("┏◚◚◚◚◚◚◚◚◚◚◚◚┓", '┃', '┃', "┗◛◛◛◛◛◛◛◛◛◛◛◛┛"),
            ("╔────────────╗", '║', '║', "╚────────────╝"),
            ("╭═────═⌘═────═╮", '│', '│', "╰═────═⌘═────═╯"),
            ("┏━────╯⌬╰────━┓", '┃', '┃', "┗━────╮⌬╭────━┛"),
            ("╔════✦❖✦════╗", '║', '║', "╚════✦❖✦════╝"),
            ("┏━┅┅┄┄⟞⟦✮⟧⟝┄┄┉┉━┓", '┃', '┃', "┗━┅┅┄┄⟞⟦✮⟧⟝┄┄┉┉━┛"),
            ("╔════░⋆ ✪ ⋆░════╗", '║', '║', "╚════░⋆ ✪ ⋆░════╝"),
            ("╭───── • ◈ • ─────╮", '│', '│', "╰───── • ◈ • ─────╯"),
            ("┯━━━━━ ●●● ━━━━━┯", '┃', '┃', "┷━━━━━ ●●● ━━━━━┷"),
            ("╔──────¤◎¤──────╗", '║', '║', "╚──────¤◎¤──────╝"),
            ("┏━━━━━━━━━━━━━━━┓", '┃', '┃', "┗━━━━━━━━━━━━━━━┛"),
            ("╭───────╯•╰───────╮", '│', '│', "╰───────╮•╭───────╯"),
            ("╒═════════════════╕", '║', '║', "╘═════════════════╛"),
            ("╔─━━━━━━ ★ ━━━━━━─╗", '║', '║', "╚─━━━━━━ ★ ━━━━━━─╝"),
            ("╭━─━─━─≪✠≫─━─━─━╮", '│', '│', "╰━─━─━─≪✠≫─━─━─━╯"),
            ("┎━─━─━─━─━─━─━─━─━┒", '┃', '┃', "┖━─━─━─━─━─━─━─━─━┚"),
            ("┍──━──━──┙◆┕──━──━──┑", '┃', '┃', "┕──━──━──┑◆┍──━──━──┙"),
            ("╔═══━━━─── • ───━━━═══╗", '║', '║', "╚═══━━━─── • ───━━━═══╝"),
            ("╔══════════•⊱✦⊰•══════════╗", '║', '║', "╚══════════•⊱✦⊰•══════════╝"),
            ("╭────────────────────────╮", '│', '│', "╰────────────────────────╯"),
            ("┏━━━━•❅•°•❈•°•❅•━━━━┓", '┃', '┃', "┗━━━━•❅•°•❈•°•❅•━━━━┛"),
            ("┎┈┈┈┈┈┈┈୨♡୧┈┈┈┈┈┈┈┒", '┃', '┃', "┖┈┈┈┈┈┈┈୨♡୧┈┈┈┈┈┈┈┚"),
            ("┏━━━━━°❀•°:🎀:°•❀°━━━━━┓", '┃', '┃', "┗━━━━━°❀•°:🎀:°•❀°━━━━━┛"),
            ("┌───── •✧✧• ─────┐", '│', '│', "└───── •✧✧• ─────┘"),
            ("╔═══════════════╗", '║', '║', "╚═══════════════╝"),
        ]
    }
}

/// Color palette for multi-color ASCII art
fn color_palette() -> Vec<fn(&str) -> ColoredString> {
    vec![
        |s| s.white(),
        |s| s.bright_white(),
        |s| s.cyan(),
        |s| s.bright_cyan(),
        |s| s.magenta(),
        |s| s.bright_magenta(),
        |s| s.yellow(),
        |s| s.bright_yellow(),
        |s| s.green(),
        |s| s.bright_green(),
    ]
}

/// Apply multi-color styling to ASCII art string
pub fn colorize_ascii_art(art: &str) -> String {
    let palette = color_palette();
    let mut result = String::new();
    let mut color_index = 0;

    for ch in art.chars() {
        if ch.is_whitespace() {
            result.push(ch);
        } else {
            let color_fn = palette[color_index % palette.len()];
            result.push_str(&color_fn(&ch.to_string()).to_string());
            color_index += 1;
        }
    }

    result
}

/// Elegant text dividers
pub struct Dividers;

impl Dividers {
    /// Get random box pattern from gist
    pub fn box_pattern() -> (&'static str, char, char, &'static str) {
        let mut rng = rand::rng();
        let boxes = AsciiArtCategories::boxes();
        boxes.choose(&mut rng).copied().unwrap_or(boxes[0])
    }
    
    /// Strip ANSI escape codes from a string to get true character width
    pub fn strip_ansi_codes(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == 'm' {
                        break;
                    }
                }
            } else {
                result.push(ch);
            }
        }
        result
    }
    
    /// Scale a decorative box pattern to fit a target width
    /// Preserves decorative elements and centers them when expanding
    /// Always returns exactly target_width characters
    fn scale_box_pattern(template: &str, target_width: usize) -> String {
        let template_chars: Vec<char> = template.chars().collect();
        if template_chars.len() < 2 {
            // Can't scale if template is too short, return as-is (but this shouldn't happen in practice)
            return template.to_string();
        }
        
        let template_width = template_chars.len();
        if target_width == template_width {
            return template.to_string();
        }
        
        let first_char = template_chars[0];
        let last_char = template_chars[template_chars.len() - 1];
        let middle: Vec<char> = template_chars[1..template_chars.len() - 1].to_vec();
        
        if middle.is_empty() {
            // No middle content, just corners - fill between them
            let fill_char = if template.contains("═") { '═' } else if template.contains("━") { '━' } else { '─' };
            let fill_width = target_width.saturating_sub(2);
            let result = format!("{}{}{}", first_char, fill_char.to_string().repeat(fill_width), last_char);
            // Verify it's exactly target_width
            let actual_width = result.chars().count();
            if actual_width != target_width {
                // Adjust fill width
                let adjusted_fill_width = fill_width as i32 + (target_width as i32 - actual_width as i32);
                if adjusted_fill_width >= 0 {
                    format!("{}{}{}", first_char, fill_char.to_string().repeat(adjusted_fill_width as usize), last_char)
                } else {
                    format!("{}{}", first_char, last_char)
                }
            } else {
                result
            }
        } else if target_width < 3 {
            // Can't have a box smaller than 2 characters (just corners)
            format!("{}{}", first_char, last_char)
        } else {
            let middle_str: String = middle.iter().collect();
            let middle_width = middle.len();
            let needed_middle_width = target_width.saturating_sub(2);
            
            if target_width < template_width {
                // Need to trim - try to preserve decorative elements in center
                // Try to keep center decorative elements
                let center_start = middle_width / 2;
                let keep_width = needed_middle_width;
                let half_keep = keep_width / 2;
                
                let start_idx = center_start.saturating_sub(half_keep);
                let end_idx = (start_idx + keep_width).min(middle_width);
                let trimmed: String = middle[start_idx..end_idx].iter().collect();
                
                // Verify the result is exactly target_width
                let result = format!("{}{}{}", first_char, trimmed, last_char);
                let actual_width = result.chars().count();
                
                if actual_width != target_width {
                    // Adjust to exact width - trim or pad as needed
                    let diff = target_width as i32 - actual_width as i32;
                    if diff > 0 {
                        // Need to add characters (shouldn't happen when trimming, but handle it)
                        let fill_char = if template.contains("═") { '═' } else if template.contains("━") { '━' } else { '─' };
                        let adjusted_centered = format!("{}{}", trimmed, fill_char.to_string().repeat(diff as usize));
                        format!("{}{}{}", first_char, adjusted_centered, last_char)
                    } else if diff < 0 {
                        // Need to trim more
                        let trim_amount = (-diff) as usize;
                        let trimmed_chars: Vec<char> = trimmed.chars().collect();
                        if trim_amount < trimmed_chars.len() {
                            let further_trimmed: String = trimmed_chars[0..trimmed_chars.len() - trim_amount].iter().collect();
                            format!("{}{}{}", first_char, further_trimmed, last_char)
                        } else {
                            format!("{}{}", first_char, last_char)
                        }
                    } else {
                        result
                    }
                } else {
                    result
                }
            } else {
                // Need to expand - center the decorative pattern
                // Find a fill character (prefer the first/last repeating char)
                let fill_char = if template.contains("═") { 
                    '═' 
                } else if template.contains("━") { 
                    '━' 
                } else if template.contains("─") {
                    '─'
                } else {
                    // Use the first or last char that appears multiple times
                    let mut char_counts: HashMap<char, usize> = HashMap::new();
                    for &ch in &middle {
                        *char_counts.entry(ch).or_insert(0) += 1;
                    }
                    char_counts.iter()
                        .max_by_key(|&(_, count)| count)
                        .map(|(&ch, _)| ch)
                        .unwrap_or('─')
                };
                
                // Center the original middle pattern
                // Calculate padding to ensure the final result is exactly target_width
                let padding_needed = needed_middle_width.saturating_sub(middle_width);
                let left_padding = padding_needed / 2;
                let right_padding = padding_needed - left_padding;
                
                // Build the centered middle section
                let left_fill = fill_char.to_string().repeat(left_padding);
                let right_fill = fill_char.to_string().repeat(right_padding);
                let centered = format!("{}{}{}", left_fill, middle_str, right_fill);
                
                // Build the result and verify it's exactly target_width
                let result = format!("{}{}{}", first_char, centered, last_char);
                let actual_width = result.chars().count();
                
                if actual_width != target_width {
                    // Adjust to exact width needed
                    let diff = target_width as i32 - actual_width as i32;
                    if diff > 0 {
                        // Need to add more characters - add to the right side of centered
                        let adjusted_centered = format!("{}{}", centered, fill_char.to_string().repeat(diff as usize));
                        format!("{}{}{}", first_char, adjusted_centered, last_char)
                    } else if diff < 0 {
                        // Need to remove characters - trim from the right side of centered
                        let trim_amount = (-diff) as usize;
                        let centered_chars: Vec<char> = centered.chars().collect();
                        if trim_amount < centered_chars.len() {
                            let trimmed: String = centered_chars[0..centered_chars.len() - trim_amount].iter().collect();
                            format!("{}{}{}", first_char, trimmed, last_char)
                        } else {
                            // Fallback: just corners
                            format!("{}{}", first_char, last_char)
                        }
                    } else {
                        result
                    }
                } else {
                    result
                }
            }
        }
    }
    
    /// Create a box with a specific pattern (or random if None)
    /// Returns (top_border, left_char, right_char, bottom_border, actual_box_width)
    pub fn create_box_with_pattern(content: &str, pattern: Option<(&'static str, char, char, &'static str)>) -> (String, char, char, String, usize) {
        let (top_template, left_char, right_char, bottom_template) = pattern.unwrap_or_else(|| Self::box_pattern());
        
        // Calculate content width (strip ANSI codes if any)
        let content_width = Self::strip_ansi_codes(content).chars().count();
        let min_width = top_template.chars().count().max(20);
        let box_width = (content_width + 2).max(min_width);
        
        // Scale both top and bottom to match box width
        let top = Self::scale_box_pattern(top_template, box_width);
        let bottom = Self::scale_box_pattern(bottom_template, box_width);
        
        // Verify the scaled patterns are exactly box_width (before colorization)
        let top_width = top.chars().count();
        let bottom_width = bottom.chars().count();
        
        // Use the actual width of the scaled pattern (should match box_width, but verify)
        let actual_box_width = top_width.max(bottom_width).max(box_width);
        
        (
            colorize_ascii_art(&top),
            left_char,
            right_char,
            colorize_ascii_art(&bottom),
            actual_box_width,
        )
    }
}

/// Helper function to render a complete box with content
pub fn render_box(content: &str, box_pattern: (&'static str, char, char, &'static str)) {
    let (box_top, left_char, right_char, box_bottom, box_width) = Dividers::create_box_with_pattern(content, Some(box_pattern));
    println!("{}", box_top);
    let content_width = Dividers::strip_ansi_codes(content).chars().count();
    let total_padding = box_width.saturating_sub(content_width + 2);
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    println!("{}{}{}{}", 
        left_char,
        " ".repeat(left_padding),
        content,
        " ".repeat(right_padding) + &right_char.to_string());
    println!("{}", box_bottom);
}

/// Color helpers for white palette
pub struct Colors;

impl Colors {
    /// Bright checking text (bright white/cyan)
    pub fn checking(text: &str) -> ColoredString {
        text.bright_white()
    }
}

/// Spinner styling with animated colors
pub fn spinner_template() -> String {
    format!("{{spinner}} \x1b[2m{{msg}}\x1b[0m")
}

/// Generate animated spinner frames with random colors and characters
pub fn spinner_frames() -> Vec<String> {
    let mut rng = rand::rng();
    let characters = vec!["✦", "✧", "⋆", "☆", "★", "✩", "✪", "✫", "✬", "✭", "✮", "✯", "✰"];
    let palette = color_palette();
    
    let mut frames = Vec::new();
    for _ in 0..8 {
        let char = characters.choose(&mut rng).unwrap_or(&"✦");
        let color_fn = palette.choose(&mut rng).unwrap();
        frames.push(color_fn(char).to_string());
    }
    
    frames
}

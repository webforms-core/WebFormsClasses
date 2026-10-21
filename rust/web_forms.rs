// web_forms.rs 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

pub mod web_forms_core {
    const GS: char = '\u{1d}';
    const US: char = '\u{1f}';
    const RS: char = '\u{1e}';

    pub struct WebForms {
        web_forms_data: String,
    }

    impl WebForms {
        pub fn new() -> Self {
            WebForms {
                web_forms_data: String::new(),
            }
        }

        pub(crate) fn add(&mut self, name: &str, value: Option<&str>) {
            if !self.web_forms_data.is_empty() {
                self.web_forms_data.push('\n');
            }
            self.web_forms_data.push_str(name);
            if let Some(v) = value {
                self.web_forms_data.push('=');
                self.web_forms_data.push_str(v);
            }
        }

        pub(crate) fn add_to_up(&mut self, name: &str, value: Option<&str>) {
            let mut line = name.to_string();
            if let Some(v) = value {
                line.push('=');
                line.push_str(v);
            }

            if !self.web_forms_data.is_empty() {
                line.push('\n');
            }

            self.web_forms_data.insert_str(0, &line);
        }

        pub(crate) fn get_line_by_index(&self, index: i32) -> String {
            if self.web_forms_data.is_empty() {
                return String::new();
            }

            let lines: Vec<&str> = self.web_forms_data.split('\n').collect();

            let mut index = index;
            if index < 0 {
                index = lines.len() as i32 + index;
            }

            if index < 0 || index >= lines.len() as i32 {
                return String::new();
            }

            lines[index as usize].to_string()
        }

        pub(crate) fn update_line_by_index(&mut self, index: i32, name: &str, value: &str) {
            if self.web_forms_data.is_empty() {
                return;
            }

            let mut lines: Vec<String> = self
                .web_forms_data
                .split('\n')
                .map(|s| s.to_string())
                .collect();

            let mut index = index;
            if index < 0 {
                index = lines.len() as i32 + index;
            }

            if index < 0 || index >= lines.len() as i32 {
                return;
            }

            let new_line = if value.is_empty() {
                name.to_string()
            } else {
                format!("{}={}", name, value)
            };
            lines[index as usize] = new_line;

            self.web_forms_data = lines.join("\n");
        }

        // For Extension
        pub fn add_line(&mut self, name: &str, value: &str) {
            self.add(name, Some(value));
        }

        // Add
        // Creates the Data if it does not exist; otherwise, Appends the New Value to the Existing Value.
        pub fn add_id(&mut self, input_place: &str, id: &str) {
            self.add(&format!("ai{}", input_place), Some(id));
        }
        pub fn add_name(&mut self, input_place: &str, name: &str) {
            self.add(&format!("an{}", input_place), Some(name));
        }
        pub fn add_value(&mut self, input_place: &str, value: &str) {
            self.add(&format!("av{}", input_place), Some(value));
        }
        pub fn add_class(&mut self, input_place: &str, class: &str) {
            self.add(&format!("ac{}", input_place), Some(class));
        }
        pub fn add_style(&mut self, input_place: &str, style: &str) {
            self.add(&format!("as{}", input_place), Some(style));
        }
        pub fn add_style_with_name_value(&mut self, input_place: &str, name: &str, value: &str) {
            self.add(
                &format!("as{}", input_place),
                Some(&format!("{}:{}", name, value)),
            );
        }
        pub fn add_option_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            selected: Option<bool>,
        ) {
            let selected_str = if selected.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("ao{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, selected_str)),
            );
        }
        pub fn add_check_box_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            checked: Option<bool>,
        ) {
            let checked_str = if checked.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("ak{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, checked_str)),
            );
        }
        pub fn add_title(&mut self, input_place: &str, title: &str) {
            self.add(&format!("al{}", input_place), Some(title));
        }
        pub fn add_label(&mut self, input_place: &str, label: &str) {
            self.add(&format!("aA{}", input_place), Some(label));
        }
        pub fn add_text(&mut self, input_place: &str, text: &str) {
            let text = text.replace("\n", "$[ln];");
            self.add(&format!("at{}", input_place), Some(&text));
        }
        pub fn add_text_to_up(&mut self, input_place: &str, text: &str) {
            let text = text.replace("\n", "$[ln];");
            self.add(&format!("pt{}", input_place), Some(&text));
        }
        pub fn add_attribute(
            &mut self,
            input_place: &str,
            attribute: &str,
            value: Option<&str>,
            splitter: Option<char>,
        ) {
            let splitter_str = match splitter {
                Some(s) if s != '\0' => s.to_string(),
                _ => String::new(),
            };
            let value_part = match value {
                Some(v) if !v.is_empty() => format!("{}{}", GS, v),
                _ => String::new(),
            };
            self.add(
                &format!("aa{}", input_place),
                Some(&format!("{}{}{}{}", attribute, GS, splitter_str, value_part)),
            );
        }
        pub fn add_tag(&mut self, input_place: &str, tag_name: &str, id: Option<&str>) {
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("{}{}", GS, i),
                _ => String::new(),
            };
            self.add(
                &format!("nt{}", input_place),
                Some(&format!("{}{}", tag_name, id_part)),
            );
        }
        pub fn add_tag_to_up(&mut self, input_place: &str, tag_name: &str, id: Option<&str>) {
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("{}{}", GS, i),
                _ => String::new(),
            };
            self.add(
                &format!("ut{}", input_place),
                Some(&format!("{}{}", tag_name, id_part)),
            );
        }
        pub fn add_tag_before(&mut self, input_place: &str, tag_name: &str, id: Option<&str>) {
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("{}{}", GS, i),
                _ => String::new(),
            };
            self.add(
                &format!("bt{}", input_place),
                Some(&format!("{}{}", tag_name, id_part)),
            );
        }
        pub fn add_tag_after(&mut self, input_place: &str, tag_name: &str, id: Option<&str>) {
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("{}{}", GS, i),
                _ => String::new(),
            };
            self.add(
                &format!("ft{}", input_place),
                Some(&format!("{}{}", tag_name, id_part)),
            );
        }
        pub fn add_hidden(
            &mut self,
            input_place: &str,
            name: &str,
            value: &str,
            id: Option<&str>,
        ) {
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("{}{}", GS, i),
                _ => String::new(),
            };
            self.add(
                &format!("ah{}", input_place),
                Some(&format!("{}{}{}{}", name, GS, value, id_part)),
            );
        }

        // Set
        // Creates the Data if it does not exist; otherwise, Replaces the Existing Value with the New Value.
        pub fn set_id(&mut self, input_place: &str, id: &str) {
            self.add(&format!("si{}", input_place), Some(id));
        }
        pub fn set_name(&mut self, input_place: &str, name: &str) {
            self.add(&format!("sn{}", input_place), Some(name));
        }
        pub fn set_value(&mut self, input_place: &str, value: &str) {
            self.add(&format!("sv{}", input_place), Some(value));
        }
        pub fn set_class(&mut self, input_place: &str, class: &str) {
            self.add(&format!("sc{}", input_place), Some(class));
        }
        pub fn set_style(&mut self, input_place: &str, style: &str) {
            self.add(&format!("ss{}", input_place), Some(style));
        }
        pub fn set_style_with_name_value(&mut self, input_place: &str, name: &str, value: &str) {
            self.add(
                &format!("ss{}", input_place),
                Some(&format!("{}:{}", name, value)),
            );
        }
        pub fn set_option_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            selected: Option<bool>,
        ) {
            let selected_str = if selected.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("so{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, selected_str)),
            );
        }
        pub fn set_checked(&mut self, input_place: &str, checked: Option<bool>) {
            let checked_str = if checked.unwrap_or(false) { "1" } else { "0" };
            self.add(&format!("sk{}", input_place), Some(checked_str));
        }
        pub fn set_check_box_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            checked: Option<bool>,
        ) {
            let checked_str = if checked.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("sk{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, checked_str)),
            );
        }
        pub fn set_title(&mut self, input_place: &str, title: &str) {
            self.add(&format!("sl{}", input_place), Some(title));
        }
        pub fn set_label(&mut self, input_place: &str, label: &str) {
            self.add(&format!("sA{}", input_place), Some(label));
        }
        pub fn set_text(&mut self, input_place: &str, text: &str) {
            let text = text.replace("\n", "$[ln];");
            self.add(&format!("st{}", input_place), Some(&text));
        }
        pub fn set_attribute(&mut self, input_place: &str, attribute: &str, value: Option<&str>) {
            let value_part = match value {
                Some(v) if !v.is_empty() => format!("{}{}", GS, v),
                _ => String::new(),
            };
            self.add(
                &format!("sa{}", input_place),
                Some(&format!("{}{}{}", attribute, GS, value_part)),
            );
        }
        pub fn set_width(&mut self, input_place: &str, width: &str) {
            self.add(&format!("sw{}", input_place), Some(width));
        }
        pub fn set_width_px(&mut self, input_place: &str, width: i32) {
            self.set_width(input_place, &format!("{}px", width));
        }
        pub fn set_height(&mut self, input_place: &str, height: &str) {
            self.add(&format!("sh{}", input_place), Some(height));
        }
        pub fn set_height_px(&mut self, input_place: &str, height: i32) {
            self.set_height(input_place, &format!("{}px", height));
        }
        pub fn set_background_color(&mut self, input_place: &str, color: &str) {
            self.add(&format!("bc{}", input_place), Some(color));
        }
        pub fn set_text_color(&mut self, input_place: &str, color: &str) {
            self.add(&format!("tc{}", input_place), Some(color));
        }
        pub fn set_font_name(&mut self, input_place: &str, name: &str) {
            self.add(&format!("fn{}", input_place), Some(name));
        }
        pub fn set_font_size(&mut self, input_place: &str, size: &str) {
            self.add(&format!("fs{}", input_place), Some(size));
        }
        pub fn set_font_size_px(&mut self, input_place: &str, size: i32) {
            self.add(&format!("fs{}", input_place), Some(&format!("{}px", size)));
        }
        pub fn set_font_bold(&mut self, input_place: &str, bold: bool) {
            self.add(&format!("fb{}", input_place), Some(if bold { "1" } else { "0" }));
        }
        pub fn set_visible(&mut self, input_place: &str, visible: bool) {
            self.add(&format!("vi{}", input_place), Some(if visible { "1" } else { "0" }));
        }
        pub fn set_text_align(&mut self, input_place: &str, align: &str) {
            self.add(&format!("ta{}", input_place), Some(align));
        }
        pub fn set_read_only(&mut self, input_place: &str, read_only: bool) {
            self.add(
                &format!("sr{}", input_place),
                Some(if read_only { "1" } else { "0" }),
            );
        }
        pub fn set_disabled(&mut self, input_place: &str, disabled: bool) {
            self.add(
                &format!("sd{}", input_place),
                Some(if disabled { "1" } else { "0" }),
            );
        }
        pub fn set_focus(&mut self, input_place: &str, focus: bool) {
            self.add(&format!("sf{}", input_place), Some(if focus { "1" } else { "0" }));
        }
        pub fn set_min_length<T: ToString>(&mut self, input_place: &str, length: T) {
            self.add(&format!("mn{}", input_place), Some(&length.to_string()));
        }
        pub fn set_max_length<T: ToString>(&mut self, input_place: &str, length: T) {
            self.add(&format!("mx{}", input_place), Some(&length.to_string()));
        }
        pub fn set_selected_value(&mut self, input_place: &str, value: &str) {
            self.add(&format!("ts{}", input_place), Some(value));
        }
        pub fn set_selected_index<T: ToString>(&mut self, input_place: &str, index: T) {
            self.add(&format!("ti{}", input_place), Some(&index.to_string()));
        }
        pub fn set_checked_value(&mut self, input_place: &str, value: &str, checked: bool) {
            self.add(
                &format!("ks{}", input_place),
                Some(&format!("{}{}{}", value, GS, if checked { "1" } else { "0" })),
            );
        }
        pub fn set_checked_index<T: ToString>(
            &mut self,
            input_place: &str,
            index: T,
            checked: bool,
        ) {
            self.add(
                &format!("ki{}", input_place),
                Some(&format!("{}{}{}", index.to_string(), GS, if checked { "1" } else { "0" })),
            );
        }
		pub fn set_custom_validity(&mut self, input_place: &str, text: &str) {
			let text = text.replace("\n", "$[ln];");
			self.add(&format!("cv{}", input_place), Some(&text));
		}

        // Insert
        // Creates the Data only if it does not exist; otherwise, does nothing.
        pub fn insert_id(&mut self, input_place: &str, id: &str) {
            self.add(&format!("ii{}", input_place), Some(id));
        }
        pub fn insert_name(&mut self, input_place: &str, name: &str) {
            self.add(&format!("in{}", input_place), Some(name));
        }
        pub fn insert_value(&mut self, input_place: &str, value: &str) {
            self.add(&format!("iv{}", input_place), Some(value));
        }
        pub fn insert_class(&mut self, input_place: &str, class: &str) {
            self.add(&format!("ic{}", input_place), Some(class));
        }
        pub fn insert_style(&mut self, input_place: &str, style: &str) {
            self.add(&format!("is{}", input_place), Some(style));
        }
        pub fn insert_style_with_name_value(&mut self, input_place: &str, name: &str, value: &str) {
            self.add(
                &format!("is{}", input_place),
                Some(&format!("{}:{}", name, value)),
            );
        }
        pub fn insert_option_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            selected: Option<bool>,
        ) {
            let selected_str = if selected.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("io{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, selected_str)),
            );
        }
        pub fn insert_check_box_tag(
            &mut self,
            input_place: &str,
            text: &str,
            value: &str,
            checked: Option<bool>,
        ) {
            let checked_str = if checked.unwrap_or(false) {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                &format!("ik{}", input_place),
                Some(&format!("{}{}{}{}", value, GS, text, checked_str)),
            );
        }
        pub fn insert_title(&mut self, input_place: &str, title: &str) {
            self.add(&format!("il{}", input_place), Some(title));
        }
        pub fn insert_label(&mut self, input_place: &str, label: &str) {
            self.add(&format!("iA{}", input_place), Some(label));
        }
        pub fn insert_text(&mut self, input_place: &str, text: &str) {
            let text = text.replace("\n", "$[ln];");
            self.add(&format!("it{}", input_place), Some(&text));
        }
        pub fn insert_attribute(
            &mut self,
            input_place: &str,
            attribute: &str,
            value: Option<&str>,
            splitter: Option<char>,
        ) {
            let splitter_str = match splitter {
                Some(s) if s != '\0' => s.to_string(),
                _ => String::new(),
            };
            let value_part = match value {
                Some(v) if !v.is_empty() => format!("{}{}", GS, v),
                _ => String::new(),
            };
            self.add(
                &format!("ia{}", input_place),
                Some(&format!("{}{}{}{}", attribute, GS, splitter_str, value_part)),
            );
        }

        // Delete
        pub fn delete_id(&mut self, input_place: &str) {
            self.add(&format!("di{}", input_place), None);
        }
        pub fn delete_name(&mut self, input_place: &str) {
            self.add(&format!("dn{}", input_place), None);
        }
        pub fn delete_value(&mut self, input_place: &str) {
            self.add(&format!("dv{}", input_place), None);
        }
        pub fn delete_class(&mut self, input_place: &str, class_name: &str) {
            self.add(&format!("dc{}", input_place), Some(class_name));
        }
        pub fn delete_style(&mut self, input_place: &str, style_name: &str) {
            self.add(&format!("ds{}", input_place), Some(style_name));
        }
        pub fn delete_option_tag(&mut self, input_place: &str, value: &str) {
            self.add(&format!("do{}", input_place), Some(value));
        }
        pub fn delete_all_option_tag(&mut self, input_place: &str) {
            self.add(&format!("do{}", input_place), Some("*"));
        }
        pub fn delete_check_box_tag(&mut self, input_place: &str, value: &str) {
            self.add(&format!("dk{}", input_place), Some(value));
        }
        pub fn delete_all_check_box_tag(&mut self, input_place: &str) {
            self.add(&format!("dk{}", input_place), Some("*"));
        }
        pub fn delete_title(&mut self, input_place: &str) {
            self.add(&format!("dl{}", input_place), None);
        }
        pub fn delete_label(&mut self, input_place: &str) {
            self.add(&format!("dA{}", input_place), None);
        }
        pub fn delete_text(&mut self, input_place: &str) {
            self.add(&format!("dt{}", input_place), None);
        }
        pub fn delete_attribute(&mut self, input_place: &str, attribute: &str) {
            self.add(&format!("da{}", input_place), Some(attribute));
        }
        pub fn delete(&mut self, input_place: &str) {
            self.add(&format!("de{}", input_place), None);
        }
        pub fn delete_parent(&mut self, input_place: &str) {
            self.add(&format!("dp{}", input_place), None);
        }

        // Tag Transformation
        pub fn swap_tag(&mut self, input_place: &str, output_place: &str) {
            self.add(&format!("sp{}", input_place), Some(output_place));
        }
        pub fn set_reflect(&mut self, input_place: &str, tag: &str) {
            self.add(&format!("sR{}", input_place), Some(tag));
        }
        pub fn set_reflect_by_output_place(&mut self, input_place: &str, output_place: &str) {
            self.add(&format!("iR{}", input_place), Some(output_place));
        }
        pub fn set_morph(&mut self, input_place: &str, tag: &str) {
            self.add(&format!("sM{}", input_place), Some(tag));
        }
        pub fn set_morph_by_output_place(&mut self, input_place: &str, output_place: &str) {
            self.add(&format!("iM{}", input_place), Some(output_place));
        }
		// The Snapshot and Rollback Mechanism is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
		// InputPlace: Only One Tag
		pub fn snapshot(&mut self, input_place: &str, key: Option<&str>, permanent: bool) {
			let key = match key {
				Some(k) if !k.is_empty() => k,
				_ => input_place,
			};
			let suffix = if permanent { format!("{}1", GS) } else { String::new() };
			self.add(
				&format!("rS{}", input_place),
				Some(&format!("{}{}", key, suffix)),
			);
		}

		pub fn rollback(&mut self, input_place: &str, key: Option<&str>, permanent: bool) {
			let key = match key {
				Some(k) if !k.is_empty() => k,
				_ => input_place,
			};
			let suffix = if permanent { format!("{}1", GS) } else { String::new() };
			self.add(
				&format!("rB{}", input_place),
				Some(&format!("{}{}", key, suffix)),
			);
		}

        // Browser
        pub fn change_url(&mut self, url: &str) {
            self.add("cu", Some(url));
        }
        pub fn set_head_title(&mut self, title: &str) {
            self.add("ht", Some(title));
        }
        pub fn clipboard_write_text(&mut self, text: &str) {
            self.add("nw", Some(text));
        }
        pub fn scroll_to(&mut self, x: &str, y: &str) {
            self.add("ws", Some(&format!("{}{}{}", x, GS, y)));
        }
        pub fn scroll_to_i32(&mut self, x: i32, y: i32) {
            self.scroll_to(&x.to_string(), &y.to_string());
        }
        pub fn history_go(&mut self, steps: &str) {
            self.add("wg", Some(steps));
        }
        pub fn history_go_i32(&mut self, steps: i32) {
            self.history_go(&steps.to_string());
        }
        pub fn reload_page(&mut self) {
            self.add("lr", None);
        }
        pub fn redirect(&mut self, path: &str) {
            self.add("lh", Some(path));
        }

        // Increase
        pub fn increase_min_length<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+n{}", input_place), Some(&value.to_string()));
        }
        pub fn increase_max_length<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+x{}", input_place), Some(&value.to_string()));
        }
        pub fn increase_font_size<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+f{}", input_place), Some(&value.to_string()));
        }
        pub fn increase_width<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+w{}", input_place), Some(&value.to_string()));
        }
        pub fn increase_height<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+h{}", input_place), Some(&value.to_string()));
        }
        pub fn increase_value<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("+v{}", input_place), Some(&value.to_string()));
        }

        // Decrease
        pub fn decrease_min_length<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-n{}", input_place), Some(&value.to_string()));
        }
        pub fn decrease_max_length<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-x{}", input_place), Some(&value.to_string()));
        }
        pub fn decrease_font_size<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-f{}", input_place), Some(&value.to_string()));
        }
        pub fn decrease_width<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-w{}", input_place), Some(&value.to_string()));
        }
        pub fn decrease_height<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-h{}", input_place), Some(&value.to_string()));
        }
        pub fn decrease_value<T: ToString>(&mut self, input_place: &str, value: T) {
            self.add(&format!("-v{}", input_place), Some(&value.to_string()));
        }

        // Event
        // ConstructorName: mouseevent, keyboardevent, uievent, focusevent, inputevent, event
        // All Method in "Event" Section Only Support Dynamic Args Once. To Support Invoking Dynamic Arguments on a Momentary Basis, Use "EventListener" Section Methods.
        pub fn trigger_event(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            constructor_name: Option<&str>,
        ) {
            let constructor_part = match constructor_name {
                Some(c) if !c.is_empty() => format!("{}{}", GS, c),
                _ => String::new(),
            };
            self.add(
                &format!("TE{}", input_place),
                Some(&format!("{}{}", html_event_listener, constructor_part)),
            );
        }
        pub fn set_post_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ep{}", input_place), Some(html_event));
        }
        pub fn set_post_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
        ) {
            self.add(
                &format!("Ep{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, output_place)),
            );
        }
        pub fn set_post_event_add_view(&mut self, input_place: &str, html_event: &str) {
            self.add(
                &format!("Ep{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, "+")),
            );
        }
        pub fn set_post_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("EP{}", input_place), Some(html_event_listener));
        }
        pub fn set_post_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
        ) {
            self.add(
                &format!("EP{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, output_place)),
            );
        }
        pub fn set_post_event_listener_add_view(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(
                &format!("EP{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, "+")),
            );
        }
        pub fn set_get_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Eg{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_get_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Eg{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_get_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EG{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        pub fn set_get_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EG{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_put_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Et{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_put_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Et{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_put_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("ET{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        pub fn set_put_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("ET{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_patch_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Ea{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_patch_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Ea{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_patch_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EA{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        pub fn set_patch_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EA{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_delete_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("El{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_delete_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("El{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_delete_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EL{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        pub fn set_delete_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EL{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_options_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Eo{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_options_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Eo{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_options_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EO{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        pub fn set_options_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EO{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, path_str, GS, output_place)),
            );
        }
        pub fn set_head_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("Eh{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path_str)),
            );
        }
        pub fn set_head_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: Option<&str>,
        ) {
            let path_str = path.unwrap_or("#");
            self.add(
                &format!("EH{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path_str)),
            );
        }
        // IsMultiPart: If this value is true, the data will be sent based on the Form and with the "content" key.
        pub fn set_send_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            data: &str,
            path: Option<&str>,
            method: Option<&str>,
            is_multi_part: Option<bool>,
            content_type: Option<&str>,
            output_place: Option<&str>,
        ) {
            let method = method.unwrap_or("POST");
            let is_multi_part = is_multi_part.unwrap_or(false);
            let content_type = content_type.unwrap_or("text/plain");
            let path_str = path.unwrap_or("#");
            let output_part = match output_place {
                Some(o) => o.to_string(),
                None => String::new(),
            };
            let data = data.replace("\n", "$[ln];").replace("\"", "$[dq];").replace("'", "$[sq];");
			let value = format!(
				"{}{}{}{}{}{}{}{}{}{}{}{}",
				html_event,
				GS,
				data,
				GS,
				path_str,
				GS,
				method,
				GS,
				if is_multi_part { "1" } else { "0" },
				GS,
				content_type,
				output_part
			);
            self.add(&format!("En{}", input_place), Some(&value));
        }
        pub fn set_send_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            data: &str,
            path: Option<&str>,
            method: Option<&str>,
            is_multi_part: Option<bool>,
            content_type: Option<&str>,
            output_place: Option<&str>,
        ) {
            let method = method.unwrap_or("POST");
            let is_multi_part = is_multi_part.unwrap_or(false);
            let content_type = content_type.unwrap_or("text/plain");
            let path_str = path.unwrap_or("#");
            let output_part = match output_place {
                Some(o) => o.to_string(),
                None => String::new(),
            };
            let data = data.replace("\n", "$[ln];");
			let value = format!(
				"{}{}{}{}{}{}{}{}{}{}{}{}",
				html_event_listener,
				GS,
				data,
				GS,
				path_str,
				GS,
				method,
				GS,
				if is_multi_part { "1" } else { "0" },
				GS,
				content_type,
				output_part
			);
            self.add(&format!("EN{}", input_place), Some(&value));
        }
        pub fn set_comment_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            index: Option<&str>,
            output_place: Option<&str>,
        ) {
            let index_str = index.unwrap_or("");
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("Eb{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event, GS, index_str, GS, output_part)),
            );
        }
        pub fn set_comment_event_i32(
            &mut self,
            input_place: &str,
            html_event: &str,
            index: i32,
            output_place: Option<&str>,
        ) {
            self.set_comment_event(input_place, html_event, Some(&index.to_string()), output_place);
        }
        pub fn set_comment_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            index: Option<&str>,
            output_place: Option<&str>,
        ) {
            let index_str = index.unwrap_or("");
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("EB{}", input_place),
                Some(&format!("{}{}{}{}{}", html_event_listener, GS, index_str, GS, output_part)),
            );
        }
        pub fn set_comment_event_listener_i32(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            index: i32,
            output_place: Option<&str>,
        ) {
            self.set_comment_event_listener(input_place, html_event_listener, Some(&index.to_string()), output_place);
        }
        pub fn set_wasm_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            wasm_language: &str,
            wasm_url: &str,
            method_name: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("[{}]", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("Ey{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    html_event,
                    GS,
                    wasm_language,
                    GS,
                    wasm_url,
                    GS,
                    method_name,
                    GS,
                    args_join,
                    output_part
                )),
            );
        }
        pub fn set_wasm_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            wasm_language: &str,
            wasm_url: &str,
            method_name: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("[{}]", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("EY{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    html_event_listener,
                    GS,
                    wasm_language,
                    GS,
                    wasm_url,
                    GS,
                    method_name,
                    GS,
                    args_join,
                    output_part
                )),
            );
        }
        pub fn set_web_socket_event(&mut self, input_place: &str, html_event: &str, path: &str) {
            self.add(
                &format!("Ew{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, path)),
            );
        }
        pub fn set_web_socket_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: &str,
        ) {
            self.add(
                &format!("EW{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, path)),
            );
        }
        pub fn set_sse_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: &str,
            should_reconnect: Option<bool>,
            reconnect_try_timeout: Option<i32>,
        ) {
            let should_reconnect = should_reconnect.unwrap_or(true);
            let reconnect_try_timeout = reconnect_try_timeout.unwrap_or(3000);
            self.add(
                &format!("Ee{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    html_event,
                    GS,
                    path,
                    GS,
                    if should_reconnect { "1" } else { "0" },
                    GS,
                    reconnect_try_timeout
                )),
            );
        }
        pub fn set_sse_event_with_output(
            &mut self,
            input_place: &str,
            html_event: &str,
            path: &str,
            output_place: &str,
            should_reconnect: Option<bool>,
            reconnect_try_timeout: Option<i32>,
        ) {
            let should_reconnect = should_reconnect.unwrap_or(true);
            let reconnect_try_timeout = reconnect_try_timeout.unwrap_or(3000);
            self.add(
                &format!("Ee{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    html_event,
                    GS,
                    path,
                    GS,
                    if should_reconnect { "1" } else { "0" },
                    GS,
                    reconnect_try_timeout,
                    GS,
                    output_place
                )),
            );
        }
        pub fn set_sse_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: &str,
            should_reconnect: Option<bool>,
            reconnect_try_timeout: Option<i32>,
        ) {
            let should_reconnect = should_reconnect.unwrap_or(true);
            let reconnect_try_timeout = reconnect_try_timeout.unwrap_or(3000);
            self.add(
                &format!("EE{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    html_event_listener,
                    GS,
                    path,
                    GS,
                    if should_reconnect { "1" } else { "0" },
                    GS,
                    reconnect_try_timeout
                )),
            );
        }
        pub fn set_sse_event_listener_with_output(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            path: &str,
            output_place: &str,
            should_reconnect: Option<bool>,
            reconnect_try_timeout: Option<i32>,
        ) {
            let should_reconnect = should_reconnect.unwrap_or(true);
            let reconnect_try_timeout = reconnect_try_timeout.unwrap_or(3000);
            self.add(
                &format!("EE{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    html_event_listener,
                    GS,
                    path,
                    GS,
                    if should_reconnect { "1" } else { "0" },
                    GS,
                    reconnect_try_timeout,
                    GS,
                    output_place
                )),
            );
        }
        pub fn set_front_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            module_path: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("Ej{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}",
                    html_event,
                    GS,
                    module_path,
                    GS,
                    output_part,
                    args_join
                )),
            );
        }
        pub fn set_front_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            module_path: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("EJ{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}",
                    html_event_listener,
                    GS,
                    module_path,
                    GS,
                    output_part,
                    args_join
                )),
            );
        }
        pub fn set_master_pages_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            output_place: Option<&str>,
        ) {
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("Eu{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, output_part)),
            );
        }
        pub fn set_master_pages_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            output_place: Option<&str>,
        ) {
            let output_part = output_place.unwrap_or("");
            self.add(
                &format!("EU{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, output_part)),
            );
        }
        pub fn set_prevent_default_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ed{}", input_place), Some(html_event));
        }
        pub fn set_prevent_default_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("ED{}", input_place), Some(html_event_listener));
        }
        pub fn set_stop_propagation_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Es{}", input_place), Some(html_event));
        }
        pub fn set_stop_propagation_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("ES{}", input_place), Some(html_event_listener));
        }
        pub fn set_method_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            method_name: &str,
            args: Option<&[String]>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add(
                &format!("Em{}", input_place),
                Some(&format!("{}{}{}{}", html_event, GS, method_name, args_join)),
            );
        }
        pub fn set_method_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            method_name: &str,
            args: Option<&[String]>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add(
                &format!("EM{}", input_place),
                Some(&format!("{}{}{}{}", html_event_listener, GS, method_name, args_join)),
            );
        }
        pub fn set_module_method_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            method_name: &str,
            args: Option<&[String]>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add(
                &format!("Ex{}", input_place),
                Some(&format!("{}{}{}{}", html_event, GS, method_name, args_join)),
            );
        }
        pub fn set_module_method_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            method_name: &str,
            args: Option<&[String]>,
        ) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add(
                &format!("EX{}", input_place),
                Some(&format!("{}{}{}{}", html_event_listener, GS, method_name, args_join)),
            );
        }
        pub fn assign_confirm_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            text: Option<&str>,
            type_: Option<&str>,
            title: Option<&str>,
            ok_text: Option<&str>,
            cancel_text: Option<&str>,
        ) {
            let text = text.unwrap_or("Are you sure you want to proceed?");
            let type_ = type_.unwrap_or("none");
            let title = title.unwrap_or("Confirm");
            let ok_text = ok_text.unwrap_or("OK");
            let cancel_text = cancel_text.unwrap_or("Cancel");

            let text_part = if text == "Are you sure you want to proceed?" { "" } else { text };
            let type_part = if type_ == "none" { "" } else { type_ };
            let title_part = if title == "Confirm" { "" } else { title };
            let ok_part = if ok_text == "OK" { "" } else { ok_text };
            let cancel_part = if cancel_text == "Cancel" { "" } else { cancel_text };

            self.add(
                &format!("Ef{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}{}",
                    html_event,
                    GS,
                    text_part,
                    GS,
                    type_part,
                    GS,
                    title_part,
                    GS,
                    ok_part,
                    GS,
                    cancel_part
                )),
            );
        }
        pub fn remove_post_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rp{}", input_place), Some(html_event));
        }
        pub fn remove_post_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RP{}", input_place), Some(html_event_listener));
        }
        pub fn remove_get_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rg{}", input_place), Some(html_event));
        }
        pub fn remove_get_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RG{}", input_place), Some(html_event_listener));
        }
        pub fn remove_put_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rt{}", input_place), Some(html_event));
        }
        pub fn remove_put_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RT{}", input_place), Some(html_event_listener));
        }
        pub fn remove_patch_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ra{}", input_place), Some(html_event));
        }
        pub fn remove_patch_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RA{}", input_place), Some(html_event_listener));
        }
        pub fn remove_delete_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rl{}", input_place), Some(html_event));
        }
        pub fn remove_delete_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RL{}", input_place), Some(html_event_listener));
        }
        pub fn remove_options_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ro{}", input_place), Some(html_event));
        }
        pub fn remove_options_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RO{}", input_place), Some(html_event_listener));
        }
        pub fn remove_head_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rh{}", input_place), Some(html_event));
        }
        pub fn remove_head_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RH{}", input_place), Some(html_event_listener));
        }
        pub fn remove_send_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rn{}", input_place), Some(html_event));
        }
        pub fn remove_send_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RN{}", input_place), Some(html_event_listener));
        }
        pub fn remove_comment_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rb{}", input_place), Some(html_event));
        }
        pub fn remove_comment_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RB{}", input_place), Some(html_event_listener));
        }
        pub fn remove_wasm_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ry{}", input_place), Some(html_event));
        }
        pub fn remove_wasm_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RY{}", input_place), Some(html_event_listener));
        }
        pub fn remove_web_socket_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rw{}", input_place), Some(html_event));
        }
        pub fn remove_web_socket_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("RW{}", input_place), Some(html_event_listener));
        }
        pub fn remove_sse_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Re{}", input_place), Some(html_event));
        }
        pub fn remove_sse_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RE{}", input_place), Some(html_event_listener));
        }
        pub fn remove_front_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rj{}", input_place), Some(html_event));
        }
        pub fn remove_front_event_listener(&mut self, input_place: &str, html_event_listener: &str) {
            self.add(&format!("RJ{}", input_place), Some(html_event_listener));
        }
        pub fn remove_prevent_default_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rd{}", input_place), Some(html_event));
        }
        pub fn remove_prevent_default_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("RD{}", input_place), Some(html_event_listener));
        }
        pub fn remove_master_pages_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Ru{}", input_place), Some(html_event));
        }
        pub fn remove_master_pages_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("RU{}", input_place), Some(html_event_listener));
        }
        pub fn remove_stop_propagation_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rs{}", input_place), Some(html_event));
        }
        pub fn remove_stop_propagation_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
        ) {
            self.add(&format!("RS{}", input_place), Some(html_event_listener));
        }
        pub fn remove_method_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            method_name: &str,
        ) {
            self.add(
                &format!("Rm{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, method_name)),
            );
        }
        pub fn remove_method_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            method_name: &str,
        ) {
            self.add(
                &format!("RM{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, method_name)),
            );
        }
        pub fn remove_module_method_event(
            &mut self,
            input_place: &str,
            html_event: &str,
            method_name: &str,
        ) {
            self.add(
                &format!("Rx{}", input_place),
                Some(&format!("{}{}{}", html_event, GS, method_name)),
            );
        }
        pub fn remove_module_method_event_listener(
            &mut self,
            input_place: &str,
            html_event_listener: &str,
            method_name: &str,
        ) {
            self.add(
                &format!("RX{}", input_place),
                Some(&format!("{}{}{}", html_event_listener, GS, method_name)),
            );
        }
        pub fn remove_confirm_event(&mut self, input_place: &str, html_event: &str) {
            self.add(&format!("Rf{}", input_place), Some(html_event));
        }

        // Custom Event
        // This Method Is Compatible With EventListener And May Not Be Compatible With Events Written As Attributes In Some Browsers.
        // Watch: attribute, style, text, children, value
        // Compare: greater, less, equal, notequal, includes, startswith, endswith, matches, changed, inrange, lengthgreater, lengthless, lengthequal
        // Range: Only Use For Compare With inrange Value. Split By Comma ","
        // Key: Only Use For Watch With attribute And style Value
        pub fn create_custom_dom_event(
            &mut self,
            input_place: &str,
            event_name: &str,
            watch: &str,
            key: &str,
            compare: &str,
            value: &str,
            range: &str,
            immediate: Option<bool>,
            delay: Option<&str>,
        ) {
            let immediate_str = if immediate.unwrap_or(false) { "1" } else { "0" };
            let delay = delay.unwrap_or("0");
            self.add(
                &format!("eC{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                    event_name,
                    GS,
                    watch,
                    GS,
                    key,
                    GS,
                    compare,
                    GS,
                    value,
                    GS,
                    range,
                    GS,
                    immediate_str,
                    GS,
                    delay
                )),
            );
        }
        pub fn create_custom_dom_event_i32(
            &mut self,
            input_place: &str,
            event_name: &str,
            watch: &str,
            key: &str,
            compare: &str,
            value: &str,
            range: &str,
            immediate: bool,
            delay: i32,
        ) {
            self.create_custom_dom_event(
                input_place,
                event_name,
                watch,
                key,
                compare,
                value,
                range,
                Some(immediate),
                Some(&delay.to_string()),
            );
        }
        pub fn enable_scroll_bottom_event(&mut self, enable: Option<bool>) {
            let enable = enable.unwrap_or(true);
            self.add("eb", Some(if enable { "1" } else { "0" }));
        }
        pub fn enable_reached_element_event(
            &mut self,
            input_place: &str,
            once: bool,
            enable: Option<bool>,
        ) {
            let enable = enable.unwrap_or(true);
            self.add(
                &format!("er{}", input_place),
                Some(&format!(
                    "{}{}{}",
                    if once { "1" } else { "0" },
                    GS,
                    if enable { "1" } else { "0" }
                )),
            );
        }

        // Module
        pub fn load_module(&mut self, module_path: &str, methods: Option<&[String]>) {
            let methods_part = match methods {
                Some(m) if !m.is_empty() => format!("{}{}[{}]", GS, "", m.join(&US.to_string())),
                _ => String::new(),
            };
            self.add("Ml", Some(&format!("{}{}", module_path, methods_part)));
        }
        pub fn unload_module(&mut self, module_path: &str) {
            self.add("Mu", Some(module_path));
        }
        pub fn delete_module_method(&mut self, method_name: &str) {
            self.add("Md", Some(method_name));
        }

        // Unit Testing
        // InputPlace Is Actual, Expected Is Tag/OutputPlace
        pub fn assert_equal(&mut self, input_place: &str, tag: &str) {
            let tag = tag.replace("\n", "$[ln];");
            self.add(&format!("At{}", input_place), Some(&tag));
        }
        pub fn assert_equal_by_output_place(&mut self, input_place: &str, output_place: &str) {
            self.add(&format!("Ao{}", input_place), Some(output_place));
        }

        // Debug
        pub fn create_debugger(&mut self, pause: Option<bool>) {
            let pause = pause.unwrap_or(false);
            self.add("Dc", Some(if pause { "1" } else { "0" }));
        }
		pub fn try_(&mut self) {
			self.add("tr", None);
		}

		pub fn catch(&mut self) {
			self.add("ca", None);
		}

		pub fn comment(&mut self, text: &str) {
			self.add("//", Some(text));
		}

        // Service Worker
        // To Use Service Worker, You Need To Add The Elanat Dedicated Module (service-worker.js) On The Client Side
        pub fn service_worker_register(
            &mut self,
            path: Option<&str>,
            scope_path: Option<&str>,
        ) {
            let path = path.unwrap_or("");
            let scope_path = scope_path.unwrap_or("");
            self.add("wR", Some(&format!("{}{}{}", path, GS, scope_path)));
        }
        pub fn service_worker_pre_cache_static(&mut self, path_list: &[String]) {
            self.add("wp", Some(&path_list.join(&GS.to_string())));
        }
        pub fn service_worker_dynamic_cache(
            &mut self,
            path: &str,
            seconds: Option<&str>,
        ) {
            let seconds = seconds.unwrap_or("");
            let seconds_part = if seconds.is_empty() {
                String::new()
            } else {
                format!("{}{}", GS, seconds)
            };
            self.add("wc", Some(&format!("{}{}", path, seconds_part)));
        }
        pub fn service_worker_dynamic_cache_i32(&mut self, path: &str, seconds: i32) {
            if seconds > 0 {
                self.service_worker_dynamic_cache(path, Some(&seconds.to_string()));
            } else {
                self.service_worker_dynamic_cache(path, Some(""));
            }
        }
        pub fn service_worker_delete_dynamic_cache(&mut self) {
            self.add("wd", None);
        }
        pub fn service_worker_delete_dynamic_cache_with_path(&mut self, path: &str) {
            self.add("wd", Some(path));
        }
        pub fn service_worker_dynamic_cache_ttl_update(
            &mut self,
            path: &str,
            seconds: Option<&str>,
        ) {
            let seconds = seconds.unwrap_or("");
            let seconds_part = if seconds.is_empty() {
                String::new()
            } else {
                format!("{}{}", GS, seconds)
            };
            self.add("wt", Some(&format!("{}{}", path, seconds_part)));
        }
        pub fn service_worker_dynamic_cache_ttl_update_i32(&mut self, path: &str, seconds: i32) {
            if seconds > 0 {
                self.service_worker_dynamic_cache_ttl_update(path, Some(&seconds.to_string()));
            } else {
                self.service_worker_dynamic_cache_ttl_update(path, Some(""));
            }
        }
        // Path: Support Wildcard Automatically And Also Support Regex If Use "re:" Before Pattern
        // Type: Type Is Cache Strategy. cachefirst, networkfirst, cacheonly, networkonly, stalerevalidate (Fast From Cache, Updates Simultaneously From The Network)
        // CacheDynamic: If True, Any Successful Network Response For That Route Will Be Stored In The Dynamic Cache
        pub fn service_worker_route_set(
            &mut self,
            path: &str,
            type_: &str,
            cache_dynamic: Option<bool>,
        ) {
            let cache_dynamic = cache_dynamic.unwrap_or(false);
            let cache_dynamic_part = if cache_dynamic {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                "wr",
                Some(&format!("{}{}{}{}", path, GS, type_, cache_dynamic_part)),
            );
        }
        pub fn service_worker_route_alias(&mut self, path: &str, to: &str) {
            self.add("wa", Some(&format!("{}{}{}", path, GS, to)));
        }
        pub fn service_worker_delete_route_alias(&mut self, path: Option<&str>) {
            if let Some(p) = path {
                self.add("wC", Some(p));
            } else {
                self.add("wC", None);
            }
        }
        // Delete All Route And Alias
        pub fn service_worker_delete_route(&mut self) {
            self.add("wD", None);
        }
        pub fn service_worker_delete_route_with_path(&mut self, path: &str) {
            self.add("wD", Some(path));
        }

        // SSE
        pub fn disconnect_sse(&mut self, path: &str) {
            self.add("Ds", Some(path));
        }
        pub fn disconnect_all_sse(&mut self) {
            self.add("Ds", None);
        }

        // State
        pub fn add_state(&mut self, path: Option<&str>, title: Option<&str>) {
            let path = path.unwrap_or("");
            let title = title.unwrap_or("");
            self.add("AS", Some(&format!("{}{}{}", path, GS, title)));
        }
        pub fn save_state(&mut self, path: Option<&str>, title: Option<&str>) {
            let path = path.unwrap_or("");
            let title = title.unwrap_or("");
            self.add("As", Some(&format!("{}{}{}", path, GS, title)));
        }
        pub fn load_state(&mut self, path: &str) {
            self.add("ls", Some(path));
        }
        pub fn delete_state(&mut self, path: Option<&str>) {
            if let Some(p) = path {
                self.add("DS", Some(p));
            } else {
                self.add("DS", None);
            }
        }
        pub fn delete_all_state(&mut self) {
            self.add("DS", Some("*"));
        }
		pub fn lock_queue(&mut self, millisecond: &str) {
			self.add("lq", Some(millisecond));
		}
		pub fn lock_queue_ms(&mut self, millisecond: i32) {
			self.lock_queue(&millisecond.to_string());
		}

        // Cookie
        pub fn set_cookie(
            &mut self,
            key: &str,
            value: &str,
            seconds: &str,
            path: Option<&str>,
        ) {
            let path_part = match path {
                Some(p) if !p.is_empty() => format!("{}{}", GS, p),
                _ => String::new(),
            };
            self.add(
                "sC",
                Some(&format!("{}{}{}{}{}{}", key, GS, value, GS, seconds, path_part)),
            );
        }
        pub fn set_cookie_i32(
            &mut self,
            key: &str,
            value: &str,
            seconds: i32,
            path: Option<&str>,
        ) {
            self.set_cookie(key, value, &seconds.to_string(), path);
        }

        // Save (Session Cache)
        pub fn save_id(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gi{}", input_place), Some(key));
        }
        pub fn save_name(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gn{}", input_place), Some(key));
        }
        pub fn save_value(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gv{}", input_place), Some(key));
        }
        pub fn save_value_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@ge{}", input_place), Some(key));
        }
        pub fn save_class(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gc{}", input_place), Some(key));
        }
        pub fn save_style(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gs{}", input_place), Some(key));
        }
        pub fn save_title(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gl{}", input_place), Some(key));
        }
        pub fn save_label(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gA{}", input_place), Some(key));
        }
        pub fn save_text(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gt{}", input_place), Some(key));
        }
        pub fn save_outer_text(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@go{}", input_place), Some(key));
        }
        pub fn save_text_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gg{}", input_place), Some(key));
        }
        pub fn save_attribute(
            &mut self,
            input_place: &str,
            attribute: &str,
            key: Option<&str>,
        ) {
            let key = key.unwrap_or(".");
            self.add(
                &format!("@ga{}", input_place),
                Some(&format!("{}{}{}", key, GS, attribute)),
            );
        }
        pub fn save_width(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gw{}", input_place), Some(key));
        }
        pub fn save_height(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gh{}", input_place), Some(key));
        }
        pub fn save_read_only(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gr{}", input_place), Some(key));
        }
        pub fn save_selected_index(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gx{}", input_place), Some(key));
        }
        pub fn save_text_align(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gT{}", input_place), Some(key));
        }
        pub fn save_node_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gL{}", input_place), Some(key));
        }
        pub fn save_visible(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gV{}", input_place), Some(key));
        }
        pub fn save_url(&mut self, url: &str, fetch_script: Option<bool>, key: Option<&str>) {
            let key = key.unwrap_or(".");
            let fetch_script = fetch_script.unwrap_or(false);
            let fetch_part = if fetch_script {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                "@gu",
                Some(&format!("{}{}{}{}", key, GS, url, fetch_part)),
            );
        }
        pub fn save_index(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@gI{}", input_place), Some(key));
        }
        pub fn remove_save(&mut self, cache_key: &str) {
            self.add("rs", Some(cache_key));
        }
        pub fn remove_all_save(&mut self) {
            self.add("rs", Some("*"));
        }
        // Calling the SetSave Method Causes Action Control Requests Triggered by Events Using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send Event, to be Temporarily Saved on the Active Page, so the Request will not be Sent to the Server Again.
        pub fn set_save(&mut self) {
            self.add("cs", Some("*"));
        }
        pub fn add_save_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("SA", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn insert_save_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("SI", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn append_save_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("SP", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn replace_save_value(&mut self, cache_key: &str, search_value: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            let search_value = search_value.replace("\n", "$[ln];");
            self.add(
                "SR",
                Some(&format!("{}{}{}{}{}", cache_key, GS, value, GS, search_value)),
            );
        }
		// Is Regex Replace
		pub fn set_format_save_value(&mut self, cache_key: &str, regex: &str, replacement: &str) {
			let replacement = replacement.replace("\n", "$[ln];");
			self.add(
				"SF",
				Some(&format!("{}{}{}{}{}", cache_key, GS, regex, GS, replacement)),
			);
		}

		// Operator: +, -, *, /, %, //, **
		pub fn set_arithmetic_save_value(&mut self, cache_key: &str, operator: &str, value: &str) {
			self.add(
				"SM",
				Some(&format!("{}{}{}{}{}", cache_key, GS, operator, GS, value)),
			);
		}
		pub fn set_arithmetic_save_value_int(&mut self, cache_key: &str, operator: &str, value: i32) {
			self.set_arithmetic_save_value(cache_key, operator, &value.to_string());
		}

		// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
		pub fn set_text_operation_save_value(
			&mut self,
			cache_key: &str,
			operation: &str,
			value1: &str,
			value2: &str,
		) {
			self.add(
				"ST",
				Some(&format!(
					"{}{}{}{}{}{}{}",
					cache_key, GS, operation, GS, value1, GS, value2
				)),
			);
		}
		pub fn set_text_operation_save_value_int(
			&mut self,
			cache_key: &str,
			operation: &str,
			value1: i32,
			value2: i32,
		) {
			self.set_text_operation_save_value(
				cache_key,
				operation,
				&value1.to_string(),
				&value2.to_string(),
			);
		}

        // Cache
        pub fn cache_id(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@ci{}", input_place), Some(key));
        }
        pub fn cache_name(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cn{}", input_place), Some(key));
        }
        pub fn cache_value(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cv{}", input_place), Some(key));
        }
        pub fn cache_value_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@ce{}", input_place), Some(key));
        }
        pub fn cache_class(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cc{}", input_place), Some(key));
        }
        pub fn cache_style(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cs{}", input_place), Some(key));
        }
        pub fn cache_title(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cl{}", input_place), Some(key));
        }
        pub fn cache_label(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cA{}", input_place), Some(key));
        }
        pub fn cache_text(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@ct{}", input_place), Some(key));
        }
        pub fn cache_outer_text(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@co{}", input_place), Some(key));
        }
        pub fn cache_text_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cg{}", input_place), Some(key));
        }
        pub fn cache_attribute(
            &mut self,
            input_place: &str,
            attribute: &str,
            key: Option<&str>,
        ) {
            let key = key.unwrap_or(".");
            self.add(
                &format!("@ca{}", input_place),
                Some(&format!("{}{}{}", key, GS, attribute)),
            );
        }
        pub fn cache_width(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cw{}", input_place), Some(key));
        }
        pub fn cache_height(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@ch{}", input_place), Some(key));
        }
        pub fn cache_read_only(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cr{}", input_place), Some(key));
        }
        pub fn cache_selected_index(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cx{}", input_place), Some(key));
        }
        pub fn cache_text_align(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cT{}", input_place), Some(key));
        }
        pub fn cache_node_length(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cL{}", input_place), Some(key));
        }
        pub fn cache_visible(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cV{}", input_place), Some(key));
        }
        pub fn cache_url(&mut self, url: &str, fetch_script: Option<bool>, key: Option<&str>) {
            let key = key.unwrap_or(".");
            let fetch_script = fetch_script.unwrap_or(false);
            let fetch_part = if fetch_script {
                format!("{}{}", GS, "1")
            } else {
                String::new()
            };
            self.add(
                "@cu",
                Some(&format!("{}{}{}{}", key, GS, url, fetch_part)),
            );
        }
        pub fn cache_index(&mut self, input_place: &str, key: Option<&str>) {
            let key = key.unwrap_or(".");
            self.add(&format!("@cI{}", input_place), Some(key));
        }
        pub fn remove_cache(&mut self, cache_key: &str) {
            self.add("rd", Some(cache_key));
        }
        pub fn remove_all_cache(&mut self) {
            self.add("rd", Some("*"));
        }
        // Calling the SetCache Method Causes Action Control Requests Triggered by events using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send event, to be Cached, so the Request will not be Sent to the Server Again.
        pub fn set_cache(&mut self, second: &str) {
            self.add("cd", Some(second));
        }
        pub fn set_cache_i32(&mut self, second: i32) {
            self.set_cache(&second.to_string());
        }
        pub fn set_cache_without_arg(&mut self) {
            self.add("cd", Some("*"));
        }
        pub fn add_cache_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("CA", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn insert_cache_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("CI", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn append_cache_value(&mut self, cache_key: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            self.add("CP", Some(&format!("{}{}{}", cache_key, GS, value)));
        }
        pub fn replace_cache_value(&mut self, cache_key: &str, search_value: &str, value: &str) {
            let value = value.replace("\n", "$[ln];");
            let search_value = search_value.replace("\n", "$[ln];");
            self.add(
                "CR",
                Some(&format!("{}{}{}{}{}", cache_key, GS, value, GS, search_value)),
            );
        }
		// Is Regex Replace
		pub fn set_format_cache_value(&mut self, cache_key: &str, regex: &str, replacement: &str) {
			let replacement = replacement.replace("\n", "$[ln];");
			self.add(
				"CF",
				Some(&format!("{}{}{}{}{}", cache_key, GS, regex, GS, replacement)),
			);
		}

		// Operator: +, -, *, /, %, //, **
		pub fn set_arithmetic_cache_value(&mut self, cache_key: &str, operator: &str, value: &str) {
			self.add(
				"CM",
				Some(&format!("{}{}{}{}{}", cache_key, GS, operator, GS, value)),
			);
		}
		pub fn set_arithmetic_cache_value_int(&mut self, cache_key: &str, operator: &str, value: i32) {
			self.set_arithmetic_cache_value(cache_key, operator, &value.to_string());
		}

		// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
		pub fn set_text_operation_cache_value(
			&mut self,
			cache_key: &str,
			operation: &str,
			value1: &str,
			value2: &str,
		) {
			self.add(
				"CT",
				Some(&format!(
					"{}{}{}{}{}{}{}",
					cache_key, GS, operation, GS, value1, GS, value2
				)),
			);
		}
		pub fn set_text_operation_cache_value_int(
			&mut self,
			cache_key: &str,
			operation: &str,
			value1: i32,
			value2: i32,
		) {
			self.set_text_operation_cache_value(
				cache_key,
				operation,
				&value1.to_string(),
				&value2.to_string(),
			);
		}

        // Call
        pub fn load_url(&mut self, input_place: &str, url: &str) {
            self.add(&format!("lu{}", input_place), Some(url));
        }
        pub fn run_action_controls(
            &mut self,
            action_controls: &str,
            without_web_forms_section: Option<bool>,
            index: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let without_web_forms_section = without_web_forms_section.unwrap_or(true);
            let index = index.unwrap_or("");
            self.add(
                "lA",
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    if without_web_forms_section { "1" } else { "0" },
                    GS,
                    index,
                    GS,
                    action_controls
                )),
            );
        }
        pub fn call_script(&mut self, script_text: &str) {
            let script_text = script_text.replace("\n", "$[ln];");
            self.add("_", Some(&script_text));
        }
        pub fn call_method(&mut self, method_name: &str, args: Option<&[String]>) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add("lm", Some(&format!("{}{}", method_name, args_join)));
        }
        pub fn call_module_method(&mut self, method_name: &str, args: Option<&[String]>) {
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            self.add("lM", Some(&format!("{}{}", method_name, args_join)));
        }
        pub fn call_post_back(
            &mut self,
            form_input_place: &str,
            output_place: Option<&str>,
        ) {
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Lp",
                Some(&format!("{}{}{}{}", "1", GS, form_input_place, output_part)),
            );
        }
        pub fn call_comment_back(
            &mut self,
            index: Option<&str>,
            input_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let index = index.unwrap_or("");
            let input_place = input_place.unwrap_or("");
            self.add(
                "LC",
                Some(&format!(
                    "{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    index,
                    GS,
                    input_place
                )),
            );
        }
        pub fn call_comment_back_i32(
            &mut self,
            index: i32,
            input_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            self.call_comment_back(Some(&index.to_string()), input_place, use_current_event);
        }
        pub fn call_wasm_back(
            &mut self,
            wasm_language: &str,
            wasm_url: &str,
            method_name: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("[{}]", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                "Ly",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    wasm_language,
                    GS,
                    wasm_url,
                    GS,
                    method_name,
                    GS,
                    args_join,
                    output_part
                )),
            );
        }
        pub fn call_web_socket_back(&mut self, path: &str, use_current_event: Option<bool>) {
            let use_current_event = use_current_event.unwrap_or(true);
            self.add(
                "Lw",
                Some(&format!(
                    "{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path
                )),
            );
        }
        pub fn call_sse_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
            should_reconnect: Option<bool>,
            reconnect_try_timeout: Option<&str>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let should_reconnect = should_reconnect.unwrap_or(true);
            let reconnect_try_timeout = reconnect_try_timeout.unwrap_or("3000");
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Ls",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    GS,
                    if should_reconnect { "1" } else { "0" },
                    GS,
                    reconnect_try_timeout,
                    output_part
                )),
            );
        }
        pub fn call_sse_back_i32(
            &mut self,
            path: &str,
            output_place: &str,
            use_current_event: bool,
            should_reconnect: bool,
            reconnect_try_timeout: i32,
        ) {
            self.call_sse_back(
                path,
                Some(output_place),
                Some(use_current_event),
                Some(should_reconnect),
                Some(&reconnect_try_timeout.to_string()),
            );
        }
        pub fn call_front(
            &mut self,
            module_path: &str,
            args: Option<&[String]>,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let args_join = match args {
                Some(a) if !a.is_empty() => format!("{}{}[{}]", GS, "", a.join(&US.to_string())),
                _ => String::new(),
            };
            let output_part = output_place.unwrap_or("");
            self.add(
                "Lj",
                Some(&format!(
                    "{}{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    module_path,
                    GS,
                    output_part,
                    args_join
                )),
            );
        }
        pub fn call_get_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Lg",
                Some(&format!(
                    "{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    output_part
                )),
            );
        }
        pub fn call_put_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Lt",
                Some(&format!(
                    "{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    output_part
                )),
            );
        }
        pub fn call_patch_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "LP",
                Some(&format!(
                    "{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    output_part
                )),
            );
        }
        pub fn call_delete_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Ld",
                Some(&format!(
                    "{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    output_part
                )),
            );
        }
        pub fn call_head_back(&mut self, path: &str, use_current_event: Option<bool>) {
            let use_current_event = use_current_event.unwrap_or(true);
            self.add(
                "Lh",
                Some(&format!(
                    "{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path
                )),
            );
        }
        pub fn call_options_back(
            &mut self,
            path: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "Lo",
                Some(&format!(
                    "{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    output_part
                )),
            );
        }
        pub fn call_send_back(
            &mut self,
            path: &str,
            method: &str,
            is_multi_part: bool,
            content_type: &str,
            data: &str,
            output_place: Option<&str>,
            use_current_event: Option<bool>,
        ) {
            let use_current_event = use_current_event.unwrap_or(true);
            let data = data.replace("\n", "$[ln];");
            let output_part = match output_place {
                Some(o) if !o.is_empty() => format!("{}{}", GS, o),
                _ => String::new(),
            };
            self.add(
                "LS",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}{}{}",
                    if use_current_event { "1" } else { "0" },
                    GS,
                    path,
                    GS,
                    method,
                    GS,
                    if is_multi_part { "1" } else { "0" },
                    GS,
                    content_type,
                    data,
                    output_part
                )),
            );
        }

        // Update
        pub fn increase(&mut self, input_place: &str, value: f32) {
            self.add(
                &format!("gt{}", input_place),
                Some(&format!("{}{}{}", "i", GS, value)),
            );
        }
        pub fn decrease(&mut self, input_place: &str, value: f32) {
            self.add(
                &format!("gt{}", input_place),
                Some(&format!("{}{}{}", "i", GS, value * -1.0)),
            );
        }
        // If You Don't Use Deep Mode, any Tags Inside the Current Tag Will Simply Be Treated as Strings. Deep Mode Does not Remove Inner Elements.
        pub fn replace(
            &mut self,
            input_place: &str,
            value: &str,
            new_value: &str,
            also_start_tag: Option<bool>,
            deep: Option<bool>,
        ) {
            let also_start_tag = also_start_tag.unwrap_or(false);
            let deep = deep.unwrap_or(true);
            self.add(
                &format!("gt{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    "r",
                    GS,
                    value,
                    GS,
                    new_value,
                    GS,
                    if also_start_tag { "1" } else { "0" },
                    GS,
                    if deep { "1" } else { "0" }
                )),
            );
        }
        // HTML Converts Attribute Names to Lowercase, so they Need to Be Written in Lowercase.
        pub fn replace_start_tag(&mut self, input_place: &str, value: &str, new_value: &str) {
            self.add(
                &format!("gt{}", input_place),
                Some(&format!("{}{}{}{}{}", "s", GS, value, GS, new_value)),
            );
        }

        // Pre Runner
        pub fn assign_delay(&mut self, milli_second: i32, index: Option<i32>) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let new_name = format!(":{}){}", milli_second, parts[0]);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }
        pub fn assign_delay_change(&mut self, milli_second: i32, index: Option<i32>) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let mut current_name = parts[0].to_string();
            if current_name.starts_with(':') && current_name.contains(')') {
                let closing_bracket = current_name.find(')').unwrap();
                current_name = current_name[closing_bracket + 1..].to_string();
            }
            let new_name = format!(":{}){}", milli_second, current_name);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }
        pub fn assign_interval(&mut self, milli_second: i32, id: Option<&str>, index: Option<i32>) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("|{}", i),
                _ => String::new(),
            };
            let new_name = format!("({}{}){}", milli_second, id_part, parts[0]);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }
        pub fn assign_interval_change(
            &mut self,
            milli_second: i32,
            id: Option<&str>,
            index: Option<i32>,
        ) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let mut current_name = parts[0].to_string();
            if current_name.starts_with('(') && current_name.contains(')') {
                let closing_bracket = current_name.find(')').unwrap();
                current_name = current_name[closing_bracket + 1..].to_string();
            }
            let id_part = match id {
                Some(i) if !i.is_empty() => format!("|{}", i),
                _ => String::new(),
            };
            let new_name = format!("({}{}){}", milli_second, id_part, current_name);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }
        pub fn delete_interval(&mut self, id: &str) {
            self.add("Di", Some(id));
        }
        pub fn assign_repeat(&mut self, count: i32, index: Option<i32>) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let new_name = format!(",{}){}", count, parts[0]);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }
        pub fn assign_repeat_change(&mut self, count: i32, index: Option<i32>) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let mut current_name = parts[0].to_string();
            if current_name.starts_with(',') && current_name.contains(')') {
                let closing_bracket = current_name.find(')').unwrap();
                current_name = current_name[closing_bracket + 1..].to_string();
            }
            let new_name = format!(",{}){}", count, current_name);
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }

        // Index
        pub fn start_index(&mut self, name: &str) {
            self.add("#", Some(name));
        }
        pub fn start_index_default(&mut self) {
            self.start_index("");
        }
        // This Index Is Automatically Run After Changing The Browser History (Back And Forward Buttons)
        pub fn start_state(&mut self) {
            self.start_index("$");
        }
        pub fn go_to(&mut self, line: &str, repeat: &str) {
            self.add("&", Some(&format!("{}{}{}", line, GS, repeat)));
        }
        pub fn go_to_i32(&mut self, line: i32, repeat: Option<i32>) {
            let repeat = repeat.unwrap_or(1);
            self.go_to(&line.to_string(), &repeat.to_string());
        }
        pub fn go_to_index(&mut self, index: &str, repeat: Option<i32>) {
            let repeat = repeat.unwrap_or(1);
            self.add("&", Some(&format!("#{}{}{}", index, GS, repeat)));
        }

        // Start
        pub fn start_transient_dom(&mut self, input_place: &str) {
            self.add("td", Some(input_place));
        }
        pub fn end_transient_dom(&mut self) {
            self.add("td", Some(";"));
        }

        // Message
        // Type: warning, problem, help, success, none
        pub fn alert(
            &mut self,
            text: &str,
            type_: Option<&str>,
            title: Option<&str>,
            ok_text: Option<&str>,
        ) {
            let type_ = type_.unwrap_or("none");
            let title = title.unwrap_or("Alert");
            let ok_text = ok_text.unwrap_or("OK");
            let type_part = if type_ == "none" { "" } else { type_ };
            let title_part = if title == "Alert" { "" } else { title };
            let ok_part = if ok_text == "OK" { "" } else { ok_text };
            self.add(
                "Al",
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    text,
                    GS,
                    type_part,
                    GS,
                    title_part,
                    GS,
                    ok_part
                )),
            );
        }
        pub fn message(
            &mut self,
            text: &str,
            type_: Option<&str>,
            duration: Option<&str>,
        ) {
            let type_ = type_.unwrap_or("none");
            let duration = duration.unwrap_or("0");
            let type_part = if type_ == "none" { "" } else { type_ };
            let duration_part = if duration == "0" { "" } else { duration };
            self.add(
                "me",
                Some(&format!(
                    "{}{}{}{}{}",
                    text,
                    GS,
                    type_part,
                    GS,
                    duration_part
                )),
            );
        }
		pub fn message_text_only(&mut self, text: &str) {
			self.message(text, None, None);
		}
        pub fn message_i32(&mut self, text: &str, type_: &str, duration: i32) {
            self.message(text, Some(type_), Some(&duration.to_string()));
        }
        pub fn message_duration_only(&mut self, text: &str, duration: i32) {
            self.message(text, Some(""), Some(&duration.to_string()));
        }
        // Type: log, info, warn, error, debug, trace, group, groupend, table
        pub fn console_message(&mut self, text: &str, type_: Option<&str>) {
            let type_ = type_.unwrap_or("log");
            let text = text.replace("\n", "$[ln];");
            let type_part = if type_ == "log" {
                String::new()
            } else {
                format!("{}{}", GS, type_)
            };
            self.add("mc", Some(&format!("{}{}", text, type_part)));
        }
        pub fn console_message_assert(&mut self, text: &str, condition: &str) {
            let text = text.replace("\n", "$[ln];");
            self.add("ma", Some(&format!("{}{}{}", text, GS, condition)));
        }

        // Enable
        //Calling The EnableWebSocket Or EnableWebSocketOnce Or AddWebSocket Methods Will Cause Any Subsequent Requests (Under WebForms Core Technology) To Operate Under The WebSocket Protocol.
        pub fn enable_web_socket(&mut self, enable: Option<bool>) {
            let enable = enable.unwrap_or(true);
            self.add("ew", Some(if enable { "1" } else { "0" }));
        }
        pub fn enable_web_socket_once(&mut self) {
            self.add("ew", Some("$"));
        }
        pub fn add_web_socket(&mut self, path: &str) {
            self.add(&format!("aw{}", path), None);
        }
        // Disconnected WebSocket
        pub fn delete_web_socket(&mut self, path: &str) {
            self.add(&format!("dw{}", path), None);
        }

        // Use
        // InputPlace Using Only For form Element
        pub fn use_web_socket(&mut self, input_place: &str) {
            self.add(&format!("uw{}", input_place), None);
        }
        pub fn use_only_change_update(&mut self, input_place: &str) {
            self.add(&format!("uo{}", input_place), None);
        }

        // Condition And Loop
        // Condition And Loop Supports Brackets and Then
        // Type: warning, problem, help, success, none
        // Interval: Value 0 is Await (if is not True, all Next Action Controls Waiting for it), Value -1 is Sync Check Once (is Support Bracket or Next Action Control), Value > 0 is Async and is Wait Based on Time Repetition Until it Becomes True (Is Support Bracket or Next Action Control, but is not Support Else).
        // Nested Conditions and Nested Loops are Possible.
        pub fn confirm_is_true_accept(
            &mut self,
            text: Option<&str>,
            type_: Option<&str>,
            title: Option<&str>,
            ok_text: Option<&str>,
            cancel_text: Option<&str>,
            interval: Option<i32>,
        ) -> &mut Self {
            let text = text.unwrap_or("Are you sure you want to proceed?");
            let type_ = type_.unwrap_or("none");
            let title = title.unwrap_or("Confirm");
            let ok_text = ok_text.unwrap_or("OK");
            let cancel_text = cancel_text.unwrap_or("Cancel");
            let interval = interval.unwrap_or(100);

            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            let text_part = if text == "Are you sure you want to proceed?" { "" } else { text };
            let type_part = if type_ == "none" { "" } else { type_ };
            let title_part = if title == "Confirm" { "" } else { title };
            let ok_part = if ok_text == "OK" { "" } else { ok_text };
            let cancel_part = if cancel_text == "Cancel" { "" } else { cancel_text };

            self.add(
                &format!("{}ct", prefix),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    text_part,
                    GS,
                    type_part,
                    GS,
                    title_part,
                    GS,
                    ok_part,
                    GS,
                    cancel_part
                )),
            );
			self
        }
        pub fn confirm_is_false_accept(
            &mut self,
            text: Option<&str>,
            type_: Option<&str>,
            title: Option<&str>,
            ok_text: Option<&str>,
            cancel_text: Option<&str>,
            interval: Option<i32>,
        ) -> &mut Self {
            let text = text.unwrap_or("Are you sure you want to proceed?");
            let type_ = type_.unwrap_or("none");
            let title = title.unwrap_or("Confirm");
            let ok_text = ok_text.unwrap_or("OK");
            let cancel_text = cancel_text.unwrap_or("Cancel");
            let interval = interval.unwrap_or(100);

            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            let text_part = if text == "Are you sure you want to proceed?" { "" } else { text };
            let type_part = if type_ == "none" { "" } else { type_ };
            let title_part = if title == "Confirm" { "" } else { title };
            let ok_part = if ok_text == "OK" { "" } else { ok_text };
            let cancel_part = if cancel_text == "Cancel" { "" } else { cancel_text };

            self.add(
                &format!("{}cf", prefix),
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    text_part,
                    GS,
                    type_part,
                    GS,
                    title_part,
                    GS,
                    ok_part,
                    GS,
                    cancel_part
                )),
            );
			self
        }
        pub fn is_greater_than(
            &mut self,
            first_value: &str,
            second_value: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}gt", prefix),
                Some(&format!("{}{}{}", first_value, GS, second_value)),
            );
			self
        }
        pub fn is_less_than(
            &mut self,
            first_value: &str,
            second_value: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}lt", prefix),
                Some(&format!("{}{}{}", first_value, GS, second_value)),
            );
			self
        }
        pub fn is_equal_to(
            &mut self,
            first_value: &str,
            second_value: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}et", prefix),
                Some(&format!("{}{}{}", first_value, GS, second_value)),
            );
			self
        }
        pub fn is_not_equal_to(
            &mut self,
            first_value: &str,
            second_value: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}Nt", prefix),
                Some(&format!("{}{}{}", first_value, GS, second_value)),
            );
			self
        }
        pub fn exist(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}ex", prefix), Some(value));
			self
        }
        pub fn not_exist(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}nx", prefix), Some(value));
			self
        }
        pub fn is_true(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}tr", prefix), Some(value));
			self
        }
        pub fn is_false(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}fa", prefix), Some(value));
			self
        }
        pub fn is_match_media(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}mm", prefix), Some(value));
			self
        }
        pub fn is_not_match_media(&mut self, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}nm", prefix), Some(value));
			self
        }
        pub fn include(&mut self, text: &str, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}In", prefix),
                Some(&format!("{}{}{}", value, GS, text)),
            );
			self
        }
        pub fn not_include(&mut self, text: &str, value: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}Nn", prefix),
                Some(&format!("{}{}{}", value, GS, text)),
            );
			self
        }
        pub fn element_exists(&mut self, input_place: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}eE", prefix), Some(input_place));
			self
        }
        pub fn element_not_exists(&mut self, input_place: &str, interval: Option<i32>) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(&format!("{}nE", prefix), Some(input_place));
			self
        }
        pub fn is_regex_match(
            &mut self,
            value: &str,
            pattern: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}re", prefix),
                Some(&format!("{}{}{}", value, GS, pattern)),
            );
			self
        }
        pub fn is_regex_not_match(
            &mut self,
            value: &str,
            pattern: &str,
            interval: Option<i32>,
        ) -> &mut Self {
            let interval = interval.unwrap_or(-1);
            let prefix = if interval >= 0 {
                format!("{{({})", interval)
            } else {
                "{".to_string()
            };
            self.add(
                &format!("{}rn", prefix),
                Some(&format!("{}{}{}", value, GS, pattern)),
            );
			self
        }
        // In: Everything Becomes A JSON List.
        // Key: Creates A Temporary Data In The Browser IndexedDB.
        // Key + "i" Creates A Temporary Data To Maintain The Loop Counter In The Browser IndexedDB.
        pub fn for_each(&mut self, path: &str, in_: &str, key: Option<&str>) -> &mut Self {
            let key = key.unwrap_or(".");
            self.add(
                "{fe",
                Some(&format!("{}{}{}{}{}", path, GS, in_, GS, key)),
            );
			self
        }
        pub fn break_(&mut self) {
            self.add(";", None);
        }
        pub fn else_(&mut self) -> &mut Self {
            self.add("}e", None);
			self
        }
        pub fn start_bracket(&mut self) {
            self.add("{", None);
        }
        pub fn end_bracket(&mut self) {
            self.add("}", None);
        }
		
		// High-Level Methods
        // Used Then In Condition And Loop Methods
        pub fn then(&mut self, new_form: WebForms) -> &mut Self {
            let data = new_form.get_web_forms_data();
            let mut new_form = new_form;
            if !data.is_empty() {
                if data.contains('\n') {
                    new_form.add_to_up("{", None);
                    new_form.add("}", None);
                }
            }
            self.append_form(&new_form);
			self
        }
        pub fn then_with<F: FnOnce(&mut WebForms)>(&mut self, configure: F) -> &mut Self {
            let mut new_form = WebForms::new();
            configure(&mut new_form);
            let data = new_form.get_web_forms_data();
            if !data.is_empty() {
                if data.contains('\n') {
                    new_form.add_to_up("{", None);
                    new_form.add("}", None);
                }
            }
            self.append_form(&new_form);
			self
        }
		pub fn repeat(&mut self, new_form: WebForms, repeat: i32) -> &mut Self {
			if repeat <= 0 {
				return self;
			}
			let body_data = new_form.get_web_forms_data();
			if body_data.is_empty() {
				return self;
			}
			let start_line = body_data.split('\n').count() as i32 * -1;
			self.append_form(&new_form);
			self.go_to(&start_line.to_string(), &(repeat - 1).to_string());
			self
		}

		pub fn repeat_with_index(&mut self, new_form: WebForms, repeat: i32, index: &str) -> &mut Self {
			if repeat <= 0 {
				return self;
			}
			self.go_to_index(index, Some(1));
			self.start_index(index);
			let body_data = new_form.get_web_forms_data();
			if body_data.is_empty() {
				return self;
			}
			self.append_form(&new_form);
			if index.is_empty() {
				let mut index_number = -1;
				for line in self.get_web_forms_data().split('\n') {
					if line.starts_with('#') {
						index_number += 1;
					}
				}
				self.go_to(&index_number.to_string(), &(repeat - 1).to_string());
			} else {
				self.go_to(index, &(repeat - 1).to_string());
			}
			self
		}
        pub fn repeat_with<F: FnOnce(&mut WebForms)>(&mut self, configure: F, repeat: i32) -> &mut Self {
            let mut new_form = WebForms::new();
            configure(&mut new_form);
            self.repeat(new_form, repeat);
			self
        }
        pub fn repeat_with_index_with<F: FnOnce(&mut WebForms)>(
            &mut self,
            configure: F,
            repeat: i32,
            index: &str,
        ) -> &mut Self {
            let mut new_form = WebForms::new();
            configure(&mut new_form);
            self.repeat_with_index(new_form, repeat, index);
			self
        }
		
		pub fn isole(&mut self, mut new_form: WebForms, input_place: &str) -> &mut Self {
			// `newForm == null` check is omitted: in Rust `WebForms` is passed by value and cannot be null.
			let body_data = new_form.get_web_forms_data();

			if body_data.is_empty() {
				return self;
			}

			let mut form = WebForms::new();
			form.start_transient_dom(input_place);

			self.append_form(&form);
			new_form.end_transient_dom();
			self.append_form(&new_form);

			self
		}

		pub fn isole_with<F: FnOnce(&mut WebForms)>(
			&mut self,
			configure: F,
			input_place: &str,
		) -> &mut Self {
			let mut new_form = WebForms::new();
			configure(&mut new_form);
			self.isole(new_form, input_place);
			self
		}

		// The Render Method is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
		// This Method Utilizes the Transient DOM; Therefore, When Selecting InputPlaces, You Must Consider the Root of the Selected InputPlace Within the Method.
		pub fn render(
			&mut self,
			mut new_form: WebForms,
			input_place: &str,
			key: Option<&str>,
			permanent: bool,
		) -> &mut Self {
			// `newForm == null` check is omitted: in Rust `WebForms` is passed by value and cannot be null.
			let body_data = new_form.get_web_forms_data();

			if body_data.is_empty() {
				return self;
			}

			let key: &str = match key {
				Some(k) if !k.is_empty() => k,
				_ => input_place,
			};

			let mut form = WebForms::new();

			let fetch_key = if permanent {
				fetch::cache(key)
			} else {
				fetch::save(key)
			};
			form.exist(&fetch_key, None);
			form.rollback(input_place, Some(key), permanent);
			form.else_();
			form.snapshot(input_place, Some(key), permanent);

			form.start_transient_dom(input_place);

			self.append_form(&form);
			new_form.end_transient_dom();
			self.append_form(&new_form);

			self
		}

		pub fn render_with<F: FnOnce(&mut WebForms)>(
			&mut self,
			configure: F,
			input_place: &str,
			key: Option<&str>,
			permanent: bool,
		) -> &mut Self {
			let mut new_form = WebForms::new();
			configure(&mut new_form);
			self.render(new_form, input_place, key, permanent);
			self
		}

        // Async
        // It Supports Brackets and Then
        pub fn async_(&mut self) -> &mut Self {
            self.add("{(a)", None);
			self
        }
        pub fn delay(&mut self, milli_second: &str) {
            self.add("De", Some(milli_second));
        }
        pub fn delay_i32(&mut self, milli_second: i32) {
            self.delay(&milli_second.to_string());
        }

        // Option
        pub fn change_option(&mut self, name: &str, value: &str) {
            self.add("co", Some(&format!("{}{}{}", name, GS, value)));
        }
        pub fn reset_option(&mut self) {
            self.add("ro", None);
        }
        pub fn reset_option_with_name(&mut self, name: &str) {
            self.add("ro", Some(name));
        }

        // Format Storage
        pub fn create_format_storage(&mut self, key: &str, data: &str) {
            self.add(".C", Some(&format!("{}{}{}", key, GS, data)));
        }
        pub fn delete_format_storage(&mut self, key: &str) {
            self.add(".D", Some(key));
        }
        pub fn add_json(&mut self, key: &str, path: &str, value: &str) {
            self.add(
                ".a",
                Some(&format!("{}{}{}{}{}{}{}", key, GS, "j", GS, value, GS, path)),
            );
        }
        // Name: For Support Attribute, Set Double At Sign (@@) Before Name.
        pub fn add_xml(
            &mut self,
            key: &str,
            path: &str,
            name: &str,
            value: Option<&str>,
        ) {
            let value = value.unwrap_or("");
            self.add(
                ".a",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "x",
                    GS,
                    name,
                    GS,
                    value,
                    GS,
                    path
                )),
            );
        }
        pub fn add_ini(
            &mut self,
            key: &str,
            path: &str,
            value: &str,
            is_ini_like: Option<bool>,
        ) {
            let is_ini_like = is_ini_like.unwrap_or(false);
            self.add(
                ".a",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "i",
                    GS,
                    if is_ini_like { "1" } else { "0" },
                    GS,
                    value,
                    GS,
                    path
                )),
            );
        }
        pub fn add_text_line(&mut self, key: &str, line: &str, text: &str) {
            self.add(
                ".a",
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "t",
                    GS,
                    text,
                    GS,
                    line
                )),
            );
        }
        pub fn add_text_line_i32(&mut self, key: &str, line: i32, text: &str) {
            self.add_text_line(key, &line.to_string(), text);
        }
        pub fn add_variable(&mut self, key: &str, value: &str) {
            self.add(
                ".a",
                Some(&format!("{}{}{}{}{}", key, GS, "v", GS, value)),
            );
        }
        pub fn update_json(&mut self, key: &str, path: &str, value: &str) {
            self.add(
                ".u",
                Some(&format!("{}{}{}{}{}{}{}", key, GS, "j", GS, value, GS, path)),
            );
        }
        pub fn update_xml(&mut self, key: &str, path: &str, value: &str) {
            self.add(
                ".u",
                Some(&format!("{}{}{}{}{}{}{}", key, GS, "x", GS, value, GS, path)),
            );
        }
        pub fn update_ini(
            &mut self,
            key: &str,
            path: &str,
            value: &str,
            is_ini_like: Option<bool>,
        ) {
            let is_ini_like = is_ini_like.unwrap_or(false);
            self.add(
                ".u",
                Some(&format!(
                    "{}{}{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "i",
                    GS,
                    if is_ini_like { "1" } else { "0" },
                    GS,
                    value,
                    GS,
                    path
                )),
            );
        }
        pub fn update_tex_line(&mut self, key: &str, line: &str, text: &str) {
            self.add(
                ".u",
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "t",
                    GS,
                    text,
                    GS,
                    line
                )),
            );
        }
        pub fn update_tex_line_i32(&mut self, key: &str, line: i32, text: &str) {
            self.update_tex_line(key, &line.to_string(), text);
        }
        pub fn update_variable(&mut self, key: &str, value: &str) {
            self.add(
                ".u",
                Some(&format!("{}{}{}{}{}", key, GS, "v", GS, value)),
            );
        }
        pub fn increase_variable(&mut self, key: &str, value: &str) {
            self.add(
                ".i",
                Some(&format!("{}{}{}{}{}", key, GS, "v", GS, value)),
            );
        }
        pub fn increase_variable_i32(&mut self, key: &str, value: i32) {
            self.increase_variable(key, &value.to_string());
        }
        pub fn decrease_variable(&mut self, key: &str, value: i32) {
            self.increase_variable(key, &(value * -1).to_string());
        }
        pub fn delete_json(&mut self, key: &str, path: &str) {
            self.add(
                ".d",
                Some(&format!("{}{}{}{}{}", key, GS, "j", GS, path)),
            );
        }
        pub fn delete_xml(&mut self, key: &str, path: &str) {
            self.add(
                ".d",
                Some(&format!("{}{}{}{}{}", key, GS, "x", GS, path)),
            );
        }
        pub fn delete_ini(
            &mut self,
            key: &str,
            path: &str,
            is_ini_like: Option<bool>,
        ) {
            let is_ini_like = is_ini_like.unwrap_or(false);
            self.add(
                ".d",
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    key,
                    GS,
                    "i",
                    GS,
                    if is_ini_like { "1" } else { "0" },
                    GS,
                    path
                )),
            );
        }
        pub fn delete_text_line(&mut self, key: &str, line: &str) {
            self.add(
                ".d",
                Some(&format!("{}{}{}{}{}", key, GS, "t", GS, line)),
            );
        }
        pub fn delete_text_line_i32(&mut self, key: &str, line: i32) {
            self.delete_text_line(key, &line.to_string());
        }
        pub fn delete_variable(&mut self, key: &str) {
            self.add(".d", Some(&format!("{}{}{}", key, GS, "v")));
        }

        // Template Engine
        // Pattern Example: {{value}}, ((value)), *value*, $value;
        pub fn bind_json_to_template(
            &mut self,
            input_place: &str,
            json_text: &str,
            path: &str,
            pattern: &str,
            also_start_tag: Option<bool>,
        ) {
            let also_start_tag = also_start_tag.unwrap_or(true);
            self.add(
                &format!("Tj{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    json_text,
                    GS,
                    path,
                    GS,
                    pattern,
                    GS,
                    if also_start_tag { "1" } else { "0" }
                )),
            );
        }
        // Because XML Elements Are Lowercased, Placeholders Must Use Lowercase Names.
        pub fn bind_xml_to_template(
            &mut self,
            input_place: &str,
            xml_text: &str,
            path: &str,
            pattern: &str,
            also_start_tag: Option<bool>,
        ) {
            let also_start_tag = also_start_tag.unwrap_or(true);
            self.add(
                &format!("Tx{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    xml_text,
                    GS,
                    path,
                    GS,
                    pattern,
                    GS,
                    if also_start_tag { "1" } else { "0" }
                )),
            );
        }
        pub fn bind_ini_to_template(
            &mut self,
            input_place: &str,
            ini_text: &str,
            path: &str,
            pattern: &str,
            also_start_tag: Option<bool>,
        ) {
            let also_start_tag = also_start_tag.unwrap_or(true);
            self.add(
                &format!("Ti{}", input_place),
                Some(&format!(
                    "{}{}{}{}{}{}{}",
                    ini_text,
                    GS,
                    path,
                    GS,
                    pattern,
                    GS,
                    if also_start_tag { "1" } else { "0" }
                )),
            );
        }

        // Inject
        // Need Add @: to First of String
        pub fn inject(&self, value: &str) -> String {
            format!("$[{}];", value)
        }

        // Action Control
        pub fn replace_action_control(
            &mut self,
            search_value: &str,
            value: &str,
            adding_to_up: Option<bool>,
        ) {
            let adding_to_up = adding_to_up.unwrap_or(false);
            if adding_to_up {
                self.add_to_up("rE", Some(&format!("{}{}{}", search_value, GS, value)));
            } else {
                self.add("rE", Some(&format!("{}{}{}", search_value, GS, value)));
            }
        }
        pub fn assign_replace(
            &mut self,
            search_value: &str,
            value: &str,
            index: Option<i32>,
        ) {
            let index = index.unwrap_or(-1);
            let current_line = self.get_line_by_index(index);
            if current_line.is_empty() {
                return;
            }
            let parts: Vec<&str> = current_line.splitn(2, '=').collect();
            let new_name = format!(
                ";{}{}{}{}{}",
                search_value,
                GS,
                value,
                GS,
                parts[0]
            );
            let new_value = if parts.len() > 1 { parts[1] } else { "" };
            self.update_line_by_index(index, &new_name, new_value);
        }

        // Hash And Checksum
        pub fn set_hash(&mut self) {
            self.add("SH", None);
        }
        pub fn set_checksum(&mut self) {
            self.add("CS", None);
        }

        pub fn checksum_calculation(&self, text: &str) -> String {
            let mut sum: i32 = 0;
            let mod_val: i32 = 65536;
            let shift: u32 = 5;

            for c in text.chars() {
                let c_val = c as i32;
                sum = ((sum.wrapping_shl(shift)) | (sum.wrapping_shr(16 - shift))) ^ c_val;
                sum %= mod_val;
            }

            sum.to_string()
        }

        pub fn get_checksum(&self) -> String {
            self.checksum_calculation(&self.get_web_forms_data())
        }

        // Get
        pub fn get_forms_action_data(&self) -> String {
            if self.web_forms_data.is_empty() {
                return String::new();
            }
            self.web_forms_data.clone()
        }

        pub fn response(&self) -> String {
            format!("[web-forms]\n{}", self.get_forms_action_data())
        }

        pub fn get_forms_action_data_line_break(&self) -> String {
            if self.web_forms_data.is_empty() {
                return String::new();
            }
            let data = self.web_forms_data.clone();
            let processed_data = data.replace("\"", "$[dq];");
            processed_data.replace("\n", "$[sln];")
        }

        // Export
        pub fn export_to_html_comment(&self, add_line: Option<bool>) -> String {
            let add_line = add_line.unwrap_or(false);
            let mut response = self.response().replace("--", "$[dd];");
            if response.ends_with('-') {
                response.pop();
                response.push_str("$[da];");
            }
            if add_line {
                format!("\n<!--{}-->", response)
            } else {
                format!("<!--{}-->", response)
            }
        }

        // Using it for SSE Response
        pub fn export_to_line_break(&self) -> String {
            format!("[web-forms]$[sln];{}", self.get_forms_action_data_line_break())
        }

        pub fn get_web_forms_data(&self) -> String {
            self.web_forms_data.clone()
        }

        pub fn append_form(&mut self, form: &WebForms) {
            let other_data = form.get_web_forms_data();
            if !other_data.is_empty() {
                if !self.web_forms_data.is_empty() {
                    self.web_forms_data.push('\n');
                }
                self.web_forms_data.push_str(&other_data);
            }
        }

        pub fn clean(&mut self) {
            self.web_forms_data.clear();
        }
    }

    pub struct Security;

    impl Security {
        pub fn safe_value(&self, value: String) -> String {
            if value.is_empty() {
                return value;
            }

            let mut value = value;
            if value.starts_with('@') {
                value = format!("@{}", value);
            }

            value = value
                .replace("\n", "$[ln];")
                .replace(",@", "$[co];@")
                .replace('\u{1c}', "")
                .replace('\u{1d}', "")
                .replace('\u{1e}', "")
                .replace('\u{1f}', "");

            value
        }
    }

    // WebForms Place Criteria (WPC) DSL
    pub mod input_place {
        pub const DOCUMENT: &str = ",";
        pub const WINDOW: &str = "`";
        // When Calling TransientDOM, Using Root will Result in the Selection of the Transient Tag.
        pub const ROOT: &str = "~";
        pub const HTML: &str = ".";
        pub const HEAD: &str = "^";
        pub const SCREEN_ORIENTATION: &str = "%";
        pub const ALL: &str = "*";
        pub const PARENT: &str = "/";
        pub const CURRENT: &str = "$";
        pub const TARGET: &str = "!";
        pub const UPPER: &str = "-";

        pub fn id(id: &str) -> String {
            id.to_string()
        }
        pub fn name(name: &str) -> String {
            format!("({})", name)
        }
        pub fn name_with_index(name: &str, index: i32) -> String {
            format!("({}){}", name, index)
        }
        pub fn all_names(name: &str) -> String {
            format!("({})*", name)
        }
        pub fn tag(tag: &str) -> String {
            format!("<{}>", tag)
        }
        pub fn tag_with_index(tag: &str, index: i32) -> String {
            format!("<{}>{}", tag, index)
        }
        pub fn all_tags(tag: &str) -> String {
            format!("<{}>*", tag)
        }
        pub fn child() -> String {
            "<>".to_string()
        }
        pub fn child_with_index(index: i32) -> String {
            format!("<>{}", index)
        }
        pub fn all_child() -> String {
            "<>*".to_string()
        }
        pub fn class(class: &str) -> String {
            format!("{{{}}}", class)
        }
        pub fn class_with_index(class: &str, index: i32) -> String {
            format!("{{{}}}{}", class, index)
        }
        pub fn all_classes(class: &str) -> String {
            format!("{{{}}}*", class)
        }
        pub fn attribute(name: &str) -> String {
            format!("\"{}\"", name)
        }
        pub fn attribute_with_index(name: &str, index: i32) -> String {
            format!("\"{}\"{}", name, index)
        }
        pub fn all_attributes(name: &str) -> String {
            format!("\"{}\"*", name)
        }
        // Operator: '^', '$', '*', '~'
        pub fn attribute_with_value(name: &str, value: &str, operator: Option<char>) -> String {
            let op_str = match operator {
                Some(op) if op != '\0' => op.to_string(),
                _ => String::new(),
            };
            format!("\"{}{}'{}\"", name, op_str, value)
        }
        pub fn attribute_with_value_index(
            name: &str,
            value: &str,
            index: i32,
            operator: Option<char>,
        ) -> String {
            let op_str = match operator {
                Some(op) if op != '\0' => op.to_string(),
                _ => String::new(),
            };
            format!("\"{}{}'{}\"{}", name, op_str, value, index)
        }
        pub fn all_attributes_with_value(
            name: &str,
            value: &str,
            operator: Option<char>,
        ) -> String {
            let op_str = match operator {
                Some(op) if op != '\0' => op.to_string(),
                _ => String::new(),
            };
            format!("\"{}{}'{}\"*", name, op_str, value)
        }
        pub fn query(query: &str) -> String {
            format!(
                "*{}",
                query.replace("=", "$[eq];").replace("|", "$[vb];").replace("?", "$[qu];")
            )
        }
        pub fn query_all(query: &str) -> String {
            format!(
                "[{}]",
                query.replace("=", "$[eq];").replace("|", "$[vb];").replace("?", "$[qu];")
            )
        }
    }

    // Do not Add any Data Before or After it
    pub mod fetch {
        use super::{RS, US};

        // Method
        pub fn random(max_value: i32) -> String {
            format!("@mr{}", max_value)
        }
        pub fn random_range(min_value: i32, max_value: i32) -> String {
            format!("@mr{}{}{}", max_value, RS, min_value)
        }
        pub fn space_to_char(text: &str, character: Option<&str>) -> String {
            let character = character.unwrap_or("-");
            format!("@sc{}{}{}", character, RS, text)
        }
        pub fn encode_uri(text: &str) -> String {
            format!("@ue{}", text)
        }
        pub fn decode_uri(text: &str) -> String {
            format!("@ud{}", text)
        }

        pub fn method(method_name: &str, args: Option<&[String]>) -> String {
            let mut return_value = format!("@cm{}", method_name);
            if let Some(a) = args {
                if !a.is_empty() {
                    return_value.push_str(&format!("{}{}", RS, a.join(&US.to_string())));
                }
            }
            return_value
        }

        pub fn module_method(method_name: &str, args: Option<&[String]>) -> String {
            let mut return_value = format!("@cM{}", method_name);
            if let Some(a) = args {
                if !a.is_empty() {
                    return_value.push_str(&format!("{}{}", RS, a.join(&US.to_string())));
                }
            }
            return_value
        }

        // MethodName: The Method Name May Need to Include the Class Name, Separated by a Period. Example: MyClassName.MyMethodName
		pub fn wasm_method(
			wasm_language: &str,
			wasm_url: &str,
			method_name: &str,
			args: Option<&[String]>,
		) -> String {
			let mut return_value = format!("@wA{}{}{}{}{}", wasm_language, RS, wasm_url, RS, method_name);
			if let Some(a) = args {
				if !a.is_empty() {
					return_value.push_str(&format!("{}{}", RS, a.join(&US.to_string())));
				}
			}
			return_value
		}

        pub fn script(script_text: &str) -> String {
            format!("@_{}", script_text.replace("\n", "$[ln];"))
        }
        pub fn load_url(url: &str, fetch_script: bool) -> String {
            format!("@lu{}{}", url, if fetch_script { format!("{}{}", RS, "1") } else { String::new() })
        }
        pub fn load_html(url: &str, fetch_input_place: &str, fetch_script: bool) -> String {
            let fetch_input_part = if fetch_input_place.is_empty() {
                String::new()
            } else {
                format!("{}{}", RS, fetch_input_place)
            };
            format!(
                "@lh{}{}{}{}",
                url,
                RS,
                if fetch_script { "1" } else { "0" },
                fetch_input_part
            )
        }
        pub fn load_line(url: &str, line: i32) -> String {
            format!("@ll{}{}{}", url, RS, line)
        }
        pub fn load_ini(url: &str, name: &str, is_ini_like: bool) -> String {
            format!(
                "@li{}{}{}{}",
                url,
                RS,
                name,
                if is_ini_like { format!("{}{}", RS, "1") } else { String::new() }
            )
        }
        // Name: Name Or Nested Paths. Is Supprt Index (Student[8].Name). Nested Paths Index Starts At 0
        pub fn load_json(url: &str, name: &str) -> String {
            format!("@lj{}{}{}", url, RS, name)
        }
        // Name: Name Or XPath; XPath Index Starts At 1
        pub fn load_xml(url: &str, name: &str) -> String {
            format!("@lx{}{}{}", url, RS, name)
        }
        // MethodName: It's Check Function Or Variable
        pub fn has_method(method_name: &str) -> String {
            format!("@hm{}", method_name)
        }
        pub fn has_module_method(method_name: &str) -> String {
            format!("@hM{}", method_name)
        }
        // This Method Return True Or False If Key Pressed
        // Modifier: Alt, AltGraph, Control, Meta, Shift, CapsLock, NumLock, ScrollLock
        pub fn get_modifier_state(modifier: &str) -> String {
            format!("@ms{}", modifier)
        }

        // Math
        pub fn math(method_name: &str, args: Option<&[String]>) -> String {
            let mut return_value = format!("@M#{}", method_name);
            if let Some(a) = args {
                if !a.is_empty() {
                    return_value.push_str(&format!("{}{}", RS, a.join(&US.to_string())));
                }
            }
            return_value
        }

        // Date
        pub const DATE_YEAR: &str = "@dy";
        // Month In JavaScript Is Start From Index 0, Month In WebForms Core Is Start From Index 1 
        pub const DATE_MONTH: &str = "@dm";
        pub const DATE_DAY: &str = "@dd";
        pub const DATE_DATE: &str = "@dD";
        pub const DATE_HOURS: &str = "@dh";
        pub const DATE_MINUTES: &str = "@di";
        pub const DATE_SECONDS: &str = "@ds";
        pub const DATE_MILLISECONDS: &str = "@dl";

        // String
        pub const SPACE: &str = "@sp";
        pub const AT_SIGN: &str = "@sa";

        // Tag
        pub fn get_id(input_place: &str) -> String {
            format!("@$i{}", input_place)
        }
        pub fn get_name(input_place: &str) -> String {
            format!("@$n{}", input_place)
        }
        pub fn get_value(input_place: &str) -> String {
            format!("@$v{}", input_place)
        }
        pub fn get_value_length(input_place: &str) -> String {
            format!("@$e{}", input_place)
        }
        pub fn get_class(input_place: &str) -> String {
            format!("@$c{}", input_place)
        }
        pub fn get_style(input_place: &str) -> String {
            format!("@$s{}", input_place)
        }
        pub fn get_title(input_place: &str) -> String {
            format!("@$l{}", input_place)
        }
        pub fn get_label(input_place: &str) -> String {
            format!("@$A{}", input_place)
        }
        pub fn get_text(input_place: &str) -> String {
            format!("@$t{}", input_place)
        }
        pub fn get_outer_text(input_place: &str) -> String {
            format!("@$o{}", input_place)
        }
        pub fn get_text_length(input_place: &str) -> String {
            format!("@$g{}", input_place)
        }
        pub fn get_attribute(input_place: &str, attribute: &str) -> String {
            format!("@$a{}{}{}", input_place, RS, attribute)
        }
        pub fn get_width(input_place: &str) -> String {
            format!("@$w{}", input_place)
        }
        pub fn get_height(input_place: &str) -> String {
            format!("@$h{}", input_place)
        }
        pub fn get_is_read_only(input_place: &str) -> String {
            format!("@$r{}", input_place)
        }
        pub fn get_selected_index(input_place: &str) -> String {
            format!("@$x{}", input_place)
        }
        pub fn get_index(input_place: &str) -> String {
            format!("@$I{}", input_place)
        }
        pub fn get_text_align(input_place: &str) -> String {
            format!("@$T{}", input_place)
        }
        pub fn get_node_length(input_place: &str) -> String {
            format!("@$L{}", input_place)
        }
        pub fn get_is_visible(input_place: &str) -> String {
            format!("@$V{}", input_place)
        }
		pub fn get_tag_hash(input_place: &str) -> String {
			format!("@$H{}", input_place)
		}
		
        // Save and Cache
        pub fn has_hash(hash: &str) -> String {
            format!("@HH{}", hash)
        }
        pub fn cookie(key: &str) -> String {
            format!("@co{}", key)
        }
        pub fn save(key: &str) -> String {
            format!("@cs{}", key)
        }
        pub fn save_with_replace(key: &str, replace_value: &str) -> String {
            format!("@cs{}{}{}", key, RS, replace_value)
        }
        pub fn save_then_remove(key: &str) -> String {
            format!("@cl{}", key)
        }
        pub fn save_length(key: &str) -> String {
            format!("@cg{}", key)
        }
        pub fn cache(key: &str) -> String {
            format!("@cd{}", key)
        }
        pub fn cache_with_replace(key: &str, replace_value: &str) -> String {
            format!("@cd{}{}{}", key, RS, replace_value)
        }
        pub fn cache_then_remove(key: &str) -> String {
            format!("@ct{}", key)
        }
        pub fn cache_length(key: &str) -> String {
            format!("@cG{}", key)
        }
        pub fn save_line(key: &str, line: i32) -> String {
            format!("@lL{}[{}", key, line)
        }
        pub fn save_line_consume(key: &str) -> String {
            format!("@lL{}", key)
        }
        // INIKey: Only Direct Key is Supported
        pub fn save_ini(key: &str, ini_key: &str) -> String {
            format!("@lI{}[{}", key, ini_key)
        }
        pub fn cache_line(key: &str, line: i32) -> String {
            format!("@dL{}[{}", key, line)
        }
        pub fn cache_line_consume(key: &str) -> String {
            format!("@dL{}", key)
        }
        // INIKey: Only Direct Key is Supported
        pub fn cache_ini(key: &str, ini_key: &str) -> String {
            format!("@dI{}[{}", key, ini_key)
        }

        // Format Storage
        pub fn format_store(key: &str) -> String {
            format!("@fr{}", key)
        }
        pub fn format_store_by_xml_query(key: &str, xpath: &str) -> String {
            format!("@fx{}{}{}", key, RS, xpath)
        }
        pub fn format_store_by_json_query(key: &str, query: &str) -> String {
            format!("@fj{}{}{}", key, RS, query)
        }
        pub fn format_store_by_ini(key: &str, name: &str) -> String {
            format!("@fi{}{}{}", key, RS, name)
        }
        pub fn format_store_by_text(key: &str, line: i32) -> String {
            format!("@ft{}{}{}", key, RS, line)
        }
        pub fn format_store_by_variable(key: &str) -> String {
            format!("@fv{}", key)
        }

        // State
        pub fn has_state(path: &str) -> String {
            format!("@hs{}", path)
        }

        // SSE
        pub fn sse_is_connected(path: &str) -> String {
            format!("@Sc{}", path)
        }

        // WebSockets
        pub fn web_sockets_is_connected(path: &str) -> String {
            format!("@Wc{}", path)
        }

        // Document
        pub const TAB_IS_ACTIVE: &str = "@da";

        // Window
        pub const HREF: &str = "@wf";
        pub const PATH_NAME: &str = "@wP";
        pub fn query(name: &str) -> String {
            format!("@wq{}", name)
        }
        pub const HASH: &str = "@wh";
        pub const HOST: &str = "@wH";
        pub const HOST_NAME: &str = "@wn";
        pub const PORT: &str = "@wT";
        pub const ORIGIN: &str = "@wo";
        pub const GET_SELECTION: &str = "@ws";
        pub const SCROLL_X: &str = "@wx";
        pub const SCROLL_Y: &str = "@wy";
        pub fn segment(index: i32) -> String {
            format!("@wS{}", index)
        }
        // It Only Works when the String Starts with the Tilde Character (~). The Path is Also Separated by the Slash Character (/). #~/Segment1/Segment2/Segment3
        pub fn hash_segment(index: i32) -> String {
            format!("@wt{}", index)
        }

        // Navigator
        pub const CLIPBOARD_TEXT: &str = "@nC";
        pub const GEO_LATITUDE: &str = "@nW";
        pub const GEO_LONGITUDE: &str = "@nO";
        pub const LANGUAGE: &str = "@nL";
        pub const IS_ON_LINE: &str = "@no";
        pub const USER_AGENT: &str = "@na";

        // Screen
        pub const SCREEN_WIDTH: &str = "@sw";
        pub const SCREEN_HEIGHT: &str = "@sh";
        pub const SCREEN_ORIENTATION_TYPE: &str = "@so";
        pub const SCREEN_ORIENTATION_ANGLE: &str = "@sr";

        // Performance
        pub const TIME_ORIGIN: &str = "@pt";
        pub const PERFORMANCE_NOW: &str = "@pn";

        // Event
        pub const EVENT: &str = "@EV";
        pub const EVENT_SERIALIZE: &str = "@Es";
        pub const EVENT_KEY: &str = "@ek";
        pub const EVENT_WHICH: &str = "@ew";
        pub const EVENT_CLIENT_X: &str = "@ex";
        pub const EVENT_CLIENT_Y: &str = "@ey";
        pub const EVENT_PAGE_X: &str = "@eX";
        pub const EVENT_PAGE_Y: &str = "@eY";
        pub const EVENT_OFFSET_X: &str = "@Ex";
        pub const EVENT_OFFSET_Y: &str = "@Ey";
        pub const EVENT_DELTA_Y: &str = "@ed";
    }

    pub mod wasm_language {
        pub const C: &str = "c";
        pub const CPP: &str = "c";
        pub const RUST: &str = "rust";
        pub const CSHARP: &str = "csharp";
        pub const GO: &str = "go";
        pub const JAVA: &str = "java";
        pub const ASSEMBLY_SCRIPT: &str = "as";
    }

    pub mod html_event {
        pub const ON_ABORT: &str = "onabort";
        pub const ON_AFTER_PRINT: &str = "onafterprint";
        pub const ON_BEFORE_PRINT: &str = "onbeforeprint";
        pub const ON_BEFORE_UNLOAD: &str = "onbeforeunload";
        pub const ON_BLUR: &str = "onblur";
        pub const ON_CAN_PLAY: &str = "oncanplay";
        pub const ON_CAN_PLAY_THROUGH: &str = "oncanplaythrough";
        pub const ON_CHANGE: &str = "onchange";
        pub const ON_CLICK: &str = "onclick";
        pub const ON_COPY: &str = "oncopy";
        pub const ON_CUT: &str = "oncut";
        pub const ON_DOUBLE_CLICK: &str = "ondblclick";
        pub const ON_DRAG: &str = "ondrag";
        pub const ON_DRAG_END: &str = "ondragend";
        pub const ON_DRAG_ENTER: &str = "ondragenter";
        pub const ON_DRAG_LEAVE: &str = "ondragleave";
        pub const ON_DRAG_OVER: &str = "ondragover";
        pub const ON_DRAG_START: &str = "ondragstart";
        pub const ON_DROP: &str = "ondrop";
        pub const ON_DURATION_CHANGE: &str = "ondurationchange";
        pub const ON_ENDED: &str = "onended";
        pub const ON_ERROR: &str = "onerror";
        pub const ON_FOCUS: &str = "onfocus";
        pub const ON_FOCUSIN: &str = "onfocusin";
        pub const ON_FOCUS_OUT: &str = "onfocusout";
        pub const ON_HASH_CHANGE: &str = "onhashchange";
        pub const ON_INPUT: &str = "oninput";
        pub const ON_INVALID: &str = "oninvalid";
        pub const ON_KEY_DOWN: &str = "onkeydown";
        pub const ON_KEY_PRESS: &str = "onkeypress";
        pub const ON_KEY_UP: &str = "onkeyup";
        pub const ON_LOAD: &str = "onload";
        pub const ON_LOADED_DATA: &str = "onloadeddata";
        pub const ON_LOADED_META_DATA: &str = "onloadedmetadata";
        pub const ON_LOAD_START: &str = "onloadstart";
        pub const ON_MOUSE_DOWN: &str = "onmousedown";
        pub const ON_MOUSE_ENTER: &str = "onmouseenter";
        pub const ON_MOUSE_LEAVE: &str = "onmouseleave";
        pub const ON_MOUSE_MOVE: &str = "onmousemove";
        pub const ON_MOUSE_OVER: &str = "onmouseover";
        pub const ON_MOUSE_OUT: &str = "onmouseout";
        pub const ON_MOUSE_UP: &str = "onmouseup";
        pub const ON_OFFLINE: &str = "onoffline";
        pub const ON_ONLINE: &str = "ononline";
        pub const ON_PAGE_HIDE: &str = "onpagehide";
        pub const ON_PAGE_SHOW: &str = "onpageshow";
        pub const ON_PASTE: &str = "onpaste";
        pub const ON_PAUSE: &str = "onpause";
        pub const ON_PLAY: &str = "onplay";
        pub const ON_PLAYING: &str = "onplaying";
        pub const ON_PROGRESS: &str = "onprogress";
        pub const ON_RATE_CHANGE: &str = "onratechange";
        pub const ON_RESIZE: &str = "onresize";
        pub const ON_RESET: &str = "onreset";
        pub const ON_SCROLL: &str = "onscroll";
        pub const ON_SEARCH: &str = "onsearch";
        pub const ON_SEEKED: &str = "onseeked";
        pub const ON_SEEKING: &str = "onseeking";
        pub const ON_SELECT: &str = "onselect";
        pub const ON_STALLED: &str = "onstalled";
        pub const ON_SUBMIT: &str = "onsubmit";
        pub const ON_SUSPEND: &str = "onsuspend";
        pub const ON_TIME_UPDATE: &str = "ontimeupdate";
        pub const ON_TOGGLE: &str = "ontoggle";
        pub const ON_TOUCH_CANCEL: &str = "ontouchcancel";
        pub const ON_TOUCHEND: &str = "ontouchend";
        pub const ON_TOUCH_MOVE: &str = "ontouchmove";
        pub const ON_TOUCH_START: &str = "ontouchstart";
        pub const ON_UNLOAD: &str = "onunload";
        pub const ON_VOLUME_CHANGE: &str = "onvolumechange";
        pub const ON_WAITING: &str = "onwaiting";
        pub const ON_WHEEL: &str = "onwheel";
    }

    pub mod html_event_listener {
        pub const ABORT: &str = "abort";
        pub const AFTER_PRINT: &str = "afterprint";
        pub const BEFORE_PRINT: &str = "beforeprint";
        pub const BEFORE_UNLOAD: &str = "beforeunload";
        pub const BLUR: &str = "blur";
        pub const CAN_PLAY: &str = "canplay";
        pub const CAN_PLAY_THROUGH: &str = "canplaythrough";
        pub const CHANGE: &str = "change";
        pub const CLICK: &str = "click";
        pub const COPY: &str = "copy";
        pub const CUT: &str = "cut";
        pub const DOUBLE_CLICK: &str = "dblclick";
        pub const DRAG: &str = "drag";
        pub const DRAG_END: &str = "dragend";
        pub const DRAG_ENTER: &str = "dragenter";
        pub const DRAG_LEAVE: &str = "dragleave";
        pub const DRAG_OVER: &str = "dragover";
        pub const DRAG_START: &str = "dragstart";
        pub const DROP: &str = "drop";
        pub const DURATION_CHANGE: &str = "durationchange";
        pub const ENDED: &str = "ended";
        pub const ERROR: &str = "error";
        pub const FOCUS: &str = "focus";
        pub const FOCUSIN: &str = "focusin";
        pub const FOCUS_OUT: &str = "focusout";
        pub const HASH_CHANGE: &str = "hashchange";
        pub const INPUT: &str = "input";
        pub const INVALID: &str = "invalid";
        pub const KEY_DOWN: &str = "keydown";
        pub const KEY_PRESS: &str = "keypress";
        pub const KEY_UP: &str = "keyup";
        pub const LOAD: &str = "load";
        pub const LOADED_DATA: &str = "loadeddata";
        pub const LOADED_META_DATA: &str = "loadedmetadata";
        pub const LOAD_START: &str = "loadstart";
        pub const MOUSE_DOWN: &str = "mousedown";
        pub const MOUSE_ENTER: &str = "mouseenter";
        pub const MOUSE_LEAVE: &str = "mouseleave";
        pub const MOUSE_MOVE: &str = "mousemove";
        pub const MOUSE_OVER: &str = "mouseover";
        pub const MOUSE_OUT: &str = "mouseout";
        pub const MOUSE_UP: &str = "mouseup";
        pub const OFFLINE: &str = "offline";
        pub const ONLINE: &str = "online";
        pub const PAGE_HIDE: &str = "pagehide";
        pub const PAGE_SHOW: &str = "pageshow";
        pub const PASTE: &str = "paste";
        pub const PAUSE: &str = "pause";
        pub const PLAY: &str = "play";
        pub const PLAYING: &str = "playing";
        pub const PROGRESS: &str = "progress";
        pub const RATE_CHANGE: &str = "ratechange";
        pub const RESIZE: &str = "resize";
        pub const RESET: &str = "reset";
        pub const SCROLL: &str = "scroll";
        pub const SEARCH: &str = "search";
        pub const SEEKED: &str = "seeked";
        pub const SEEKING: &str = "seeking";
        pub const SELECT: &str = "select";
        pub const STALLED: &str = "stalled";
        pub const SUBMIT: &str = "submit";
        pub const SUSPEND: &str = "suspend";
        pub const TIME_UPDATE: &str = "timeupdate";
        pub const TOGGLE: &str = "toggle";
        pub const TOUCH_CANCEL: &str = "touchcancel";
        pub const TOUCHEND: &str = "touchend";
        pub const TOUCH_MOVE: &str = "touchmove";
        pub const TOUCH_START: &str = "touchstart";
        pub const UNLOAD: &str = "unload";
        pub const VOLUME_CHANGE: &str = "volumechange";
        pub const WAITING: &str = "waiting";
        pub const WHEEL: &str = "wheel";

        pub const ANIMATION_END: &str = "animationend";
        pub const ANIMATION_ITERATION: &str = "animationiteration";
        pub const ANIMATION_START: &str = "animationstart";
        pub const CONTEXT_MENU: &str = "contextmenu";
        pub const FULL_SCREEN_CHANGE: &str = "fullscreenchange";
        pub const FULL_SCREEN_ERROR: &str = "fullscreenerror";
        pub const POP_STATE: &str = "popstate";
        pub const TRANSITION_END: &str = "transitionend";
        pub const STORAGE: &str = "storage";

        // Custom
        pub const SCROLL_BOTTOM: &str = "scrollbottom"; // Need Call EnableScrollBottomEvent Method Before
        pub const ELEMENT_REACHED: &str = "elementreached"; // Need Call EnableReachedElementEvent Method Before
    }

    pub trait WebFormsStringExt {
        fn child(&self, value: &str) -> String;
        fn parent(&self) -> String;
        fn criteria(&self, value: &str) -> String;
        fn append_fetch_replace(&self, search_value: &str, value: &str) -> String;
        fn line_break(&self, encode_line: Option<bool>) -> String;
        fn to_js_string(&self) -> String;
        fn to_js_object(&self) -> String;
        fn to_js_return_object(&self) -> String;
    }

    impl WebFormsStringExt for str {
        fn child(&self, value: &str) -> String {
            if self.is_empty() {
                return value.to_string();
            }
            format!("{}|{}", self, value)
        }

        fn parent(&self) -> String {
            if self.is_empty() {
                return self.to_string();
            }
            if self.ends_with("|/") || self.ends_with("//") {
                return format!("{}/", self);
            }
            format!("{}|/", self)
        }

        fn criteria(&self, value: &str) -> String {
            if self.is_empty() {
                return value.to_string();
            }
            format!(
                "{}?{}",
                self,
                value.replace("|", "$[vb];").replace("?", "$[qu];").replace("=", "$[eq];")
            )
        }

		fn append_fetch_replace(&self, search_value: &str, value: &str) -> String {
			const FS: char = '\u{1c}';
			let text = if self.len() > 0 { &self[1..] } else { self };
			format!("@;{}{}{}{}{}", search_value, FS, value, FS, text)
		}

        fn line_break(&self, encode_line: Option<bool>) -> String {
            let encode = if encode_line.unwrap_or(false) {
                "$[sln];"
            } else {
                ""
            };
            self.replace("\r\n", encode)
                .replace("\n", encode)
                .replace("\r", encode)
        }

        // Converts Numbers to Strings
        fn to_js_string(&self) -> String {
            format!("\"{}\"", self)
        }

        // Get JS Object Momentary 
        fn to_js_object(&self) -> String {
            format!("${}", self)
        }

        // Get JS Object Returned Value Once
        fn to_js_return_object(&self) -> String {
            format!("$@{}", self)
        }
    }
}

pub use web_forms_core::*;

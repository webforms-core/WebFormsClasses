# web_forms.rb 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
# Compatible with WebFormsJS version 2.2

module WebFormsCore
  class WebForms
    GS = 29.chr
    US = 31.chr

    def initialize
      @web_forms_data = ""
    end

    def add(name, value = nil)
      if @web_forms_data.length > 0
        @web_forms_data << "\n"
      end

      @web_forms_data << name
      unless value.nil?
        @web_forms_data << "="
        @web_forms_data << value
      end
    end

    def add_to_up(name, value = nil)
      line = value.nil? ? name : "#{name}=#{value}"

      if @web_forms_data.length > 0
        line += "\n"
      end

      @web_forms_data.prepend(line)
    end

    def get_line_by_index(index)
      return "" if @web_forms_data.length == 0

      data = @web_forms_data
      lines = data.split("\n", -1)

      index = lines.length + index if index < 0

      return "" if index < 0 || index >= lines.length

      lines[index]
    end

    def update_line_by_index(index, name, value)
      return if @web_forms_data.length == 0

      data = @web_forms_data
      lines = data.split("\n", -1)

      index = lines.length + index if index < 0

      return if index < 0 || index >= lines.length

      lines[index] = name + (value.nil? || value.empty? ? "" : "=" + value)

      @web_forms_data.clear
      @web_forms_data << lines.join("\n")
    end

    # For Extension
    def add_line(name, value)
      add(name, value)
    end

    # Add
    # Creates the Data if it does not exist; otherwise, Appends the New Value to the Existing Value.
    def add_id(input_place, id)
      add("ai" + input_place, id)
    end

    def add_name(input_place, name)
      add("an" + input_place, name)
    end

    def add_value(input_place, value)
      add("av" + input_place, value)
    end

    def add_class(input_place, class_name)
      add("ac" + input_place, class_name)
    end

    def add_style(input_place, name_or_style, value = nil)
      if value.nil?
        add("as" + input_place, name_or_style)
      else
        add("as" + input_place, name_or_style + ':' + value)
      end
    end

    def add_option_tag(input_place, text, value, selected = false)
      add("ao" + input_place, value + GS + text + (selected ? GS + "1" : ""))
    end

    def add_check_box_tag(input_place, text, value, checked = false)
      add("ak" + input_place, value + GS + text + (checked ? GS + "1" : ""))
    end

    def add_title(input_place, title)
      add("al" + input_place, title)
    end

    def add_label(input_place, label)
      add("aA" + input_place, label)
    end

    def add_text(input_place, text)
      add("at" + input_place, text.gsub("\n", "$[ln];"))
    end

    def add_text_to_up(input_place, text)
      add("pt" + input_place, text.gsub("\n", "$[ln];"))
    end

    def add_attribute(input_place, attribute, value = "", splitter = "\0")
      add("aa" + input_place, attribute + GS + (splitter != "\0" ? splitter.to_s : "") + (!value.nil? && !value.empty? ? GS + value : ""))
    end

    def add_tag(input_place, tag_name, id = "")
      add("nt" + input_place, tag_name + (!id.nil? && !id.empty? ? GS + id : ""))
    end

    def add_tag_to_up(input_place, tag_name, id = "")
      add("ut" + input_place, tag_name + (!id.nil? && !id.empty? ? GS + id : ""))
    end

    def add_tag_before(input_place, tag_name, id = "")
      add("bt" + input_place, tag_name + (!id.nil? && !id.empty? ? GS + id : ""))
    end

    def add_tag_after(input_place, tag_name, id = "")
      add("ft" + input_place, tag_name + (!id.nil? && !id.empty? ? GS + id : ""))
    end

    def add_hidden(input_place, name, value, id = "")
      add("ah" + input_place, name + GS + value + (!id.nil? && !id.empty? ? GS + id : ""))
    end

    # Set
    # Creates the Data if it does not exist; otherwise, Replaces the Existing Value with the New Value.
    def set_id(input_place, id)
      add("si" + input_place, id)
    end

    def set_name(input_place, name)
      add("sn" + input_place, name)
    end

    def set_value(input_place, value)
      add("sv" + input_place, value)
    end

    def set_class(input_place, class_name)
      add("sc" + input_place, class_name)
    end

    def set_style(input_place, name_or_style, value = nil)
      if value.nil?
        add("ss" + input_place, name_or_style)
      else
        add("ss" + input_place, name_or_style + ':' + value)
      end
    end

    def set_option_tag(input_place, text, value, selected = false)
      add("so" + input_place, value + GS + text + (selected ? GS + "1" : ""))
    end

    def set_checked(input_place, checked = false)
      add("sk" + input_place, checked ? "1" : "0")
    end

    def set_check_box_tag(input_place, text, value, checked = false)
      add("sk" + input_place, value + GS + text + (checked ? GS + "1" : ""))
    end

    def set_title(input_place, title)
      add("sl" + input_place, title)
    end

    def set_label(input_place, label)
      add("sA" + input_place, label)
    end

    def set_text(input_place, text)
      add("st" + input_place, text.gsub("\n", "$[ln];"))
    end

    def set_attribute(input_place, attribute, value = "")
      add("sa" + input_place, attribute + GS + (!value.nil? && !value.empty? ? GS + value : ""))
    end

    def set_width(input_place, width)
      final_width = width.is_a?(Integer) ? "#{width}px" : width.to_s
      add("sw" + input_place, final_width)
    end

    def set_height(input_place, height)
      final_height = height.is_a?(Integer) ? "#{height}px" : height.to_s
      add("sh" + input_place, final_height)
    end

    def set_background_color(input_place, color)
      add("bc" + input_place, color)
    end

    def set_text_color(input_place, color)
      add("tc" + input_place, color)
    end

    def set_font_name(input_place, name)
      add("fn" + input_place, name)
    end

    def set_font_size(input_place, size)
      final_size = size.is_a?(Integer) ? "#{size}px" : size.to_s
      add("fs" + input_place, final_size)
    end

    def set_font_bold(input_place, bold)
      add("fb" + input_place, bold ? "1" : "0")
    end

    def set_visible(input_place, visible)
      add("vi" + input_place, visible ? "1" : "0")
    end

    def set_text_align(input_place, align)
      add("ta" + input_place, align)
    end

    def set_read_only(input_place, read_only)
      add("sr" + input_place, read_only ? "1" : "0")
    end

    def set_disabled(input_place, disabled)
      add("sd" + input_place, disabled ? "1" : "0")
    end

    def set_focus(input_place, focus)
      add("sf" + input_place, focus ? "1" : "0")
    end

    def set_min_length(input_place, length)
      add("mn" + input_place, length.to_s)
    end

    def set_max_length(input_place, length)
      add("mx" + input_place, length.to_s)
    end

    def set_selected_value(input_place, value)
      add("ts" + input_place, value)
    end

    def set_selected_index(input_place, index)
      add("ti" + input_place, index.to_s)
    end

    def set_checked_value(input_place, value, checked)
      add("ks" + input_place, value + GS + (checked ? "1" : "0"))
    end

    def set_checked_index(input_place, index, checked)
      add("ki" + input_place, index.to_s + GS + (checked ? "1" : "0"))
    end

	def set_custom_validity(input_place, text)
	  add("cv" + input_place, text.gsub("\n", "$[ln];"))
	end

    # Insert
    # Creates the Data only if it does not exist; otherwise, does nothing.
    def insert_id(input_place, id)
      add("ii" + input_place, id)
    end

    def insert_name(input_place, name)
      add("in" + input_place, name)
    end

    def insert_value(input_place, value)
      add("iv" + input_place, value)
    end

    def insert_class(input_place, class_name)
      add("ic" + input_place, class_name)
    end

    def insert_style(input_place, name_or_style, value = nil)
      if value.nil?
        add("is" + input_place, name_or_style)
      else
        add("is" + input_place, name_or_style + ':' + value)
      end
    end

    def insert_option_tag(input_place, text, value, selected = false)
      add("io" + input_place, value + GS + text + (selected ? GS + "1" : ""))
    end

    def insert_check_box_tag(input_place, text, value, checked = false)
      add("ik" + input_place, value + GS + text + (checked ? GS + "1" : ""))
    end

    def insert_title(input_place, title)
      add("il" + input_place, title)
    end

    def insert_label(input_place, label)
      add("iA" + input_place, label)
    end

    def insert_text(input_place, text)
      add("it" + input_place, text.gsub("\n", "$[ln];"))
    end

    def insert_attribute(input_place, attribute, value = "", splitter = "\0")
      add("ia" + input_place, attribute + GS + (splitter != "\0" ? splitter.to_s : "") + (!value.nil? && !value.empty? ? GS + value : ""))
    end

    # Delete
    def delete_id(input_place)
      add("di" + input_place)
    end

    def delete_name(input_place)
      add("dn" + input_place)
    end

    def delete_value(input_place)
      add("dv" + input_place)
    end

    def delete_class(input_place, class_name)
      add("dc" + input_place, class_name)
    end

    def delete_style(input_place, style_name)
      add("ds" + input_place, style_name)
    end

    def delete_option_tag(input_place, value)
      add("do" + input_place, value)
    end

    def delete_all_option_tag(input_place)
      add("do" + input_place, "*")
    end

    def delete_check_box_tag(input_place, value)
      add("dk" + input_place, value)
    end

    def delete_all_check_box_tag(input_place)
      add("dk" + input_place, "*")
    end

    def delete_title(input_place)
      add("dl" + input_place)
    end

    def delete_label(input_place)
      add("dA" + input_place)
    end

    def delete_text(input_place)
      add("dt" + input_place)
    end

    def delete_attribute(input_place, attribute)
      add("da" + input_place, attribute)
    end

    def delete(input_place)
      add("de" + input_place)
    end

    def delete_parent(input_place)
      add("dp" + input_place)
    end

    # Tag Transformation
    def swap_tag(input_place, output_place)
      add("sp" + input_place, output_place)
    end

    def set_reflect(input_place, tag)
      add("sR" + input_place, tag)
    end

    def set_reflect_by_output_place(input_place, output_place)
      add("iR" + input_place, output_place)
    end

    def set_morph(input_place, tag)
      add("sM" + input_place, tag)
    end

    def set_morph_by_output_place(input_place, output_place)
      add("iM" + input_place, output_place)
    end

	def snapshot(input_place, key = "", permanent = false)
	  add("rS" + input_place, ((key.nil? || key.empty?) ? input_place : key) + (permanent ? GS + "1" : ""))
	end

	def rollback(input_place, key = "", permanent = false)
	  add("rB" + input_place, ((key.nil? || key.empty?) ? input_place : key) + (permanent ? GS + "1" : ""))
	end

    # Browser
    def change_url(url)
      add("cu", url)
    end

    def set_head_title(title)
      add("ht", title)
    end

    def clipboard_write_text(text)
      add("nw", text)
    end

    def scroll_to(x, y)
      add("ws", x.to_s + GS + y.to_s)
    end

    def history_go(steps)
      add("wg", steps.to_s)
    end

    def reload_page
      add("lr")
    end

    def redirect(path)
      add("lh", path)
    end

    # Increase
    def increase_min_length(input_place, value)
      add("+n" + input_place, value.to_s)
    end

    def increase_max_length(input_place, value)
      add("+x" + input_place, value.to_s)
    end

    def increase_font_size(input_place, value)
      add("+f" + input_place, value.to_s)
    end

    def increase_width(input_place, value)
      add("+w" + input_place, value.to_s)
    end

    def increase_height(input_place, value)
      add("+h" + input_place, value.to_s)
    end

    def increase_value(input_place, value)
      add("+v" + input_place, value.to_s)
    end

    # Decrease
    def decrease_min_length(input_place, value)
      add("-n" + input_place, value.to_s)
    end

    def decrease_max_length(input_place, value)
      add("-x" + input_place, value.to_s)
    end

    def decrease_font_size(input_place, value)
      add("-f" + input_place, value.to_s)
    end

    def decrease_width(input_place, value)
      add("-w" + input_place, value.to_s)
    end

    def decrease_height(input_place, value)
      add("-h" + input_place, value.to_s)
    end

    def decrease_value(input_place, value)
      add("-v" + input_place, value.to_s)
    end

    # Event
    # ConstructorName: mouseevent, keyboardevent, uievent, focusevent, inputevent, event
    # All Method in "Event" Section Only Support Dynamic Args Once. To Support Invoking Dynamic Arguments on a Momentary Basis, Use "EventListener" Section Methods.
    def trigger_event(input_place, html_event_listener, constructor_name = nil)
      add("TE" + input_place, html_event_listener + (!constructor_name.nil? && !constructor_name.empty? ? GS + constructor_name : ""))
    end

    def set_post_event(input_place, html_event, output_place = nil)
      if output_place.nil?
        add("Ep" + input_place, html_event)
      elsif output_place == "+"
        add("Ep" + input_place, html_event + GS + "+")
      else
        add("Ep" + input_place, html_event + GS + output_place)
      end
    end

    def set_post_event_listener(input_place, html_event_listener, output_place = nil)
      if output_place.nil?
        add("EP" + input_place, html_event_listener)
      elsif output_place == "+"
        add("EP" + input_place, html_event_listener + GS + "+")
      else
        add("EP" + input_place, html_event_listener + GS + output_place)
      end
    end

    def set_get_event(input_place, html_event, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("Eg" + input_place, html_event + GS + path)
      else
        add("Eg" + input_place, html_event + GS + path + GS + output_place)
      end
    end

    def set_get_event_listener(input_place, html_event_listener, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("EG" + input_place, html_event_listener + GS + path)
      else
        add("EG" + input_place, html_event_listener + GS + path + GS + output_place)
      end
    end

    def set_put_event(input_place, html_event, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("Et" + input_place, html_event + GS + path)
      else
        add("Et" + input_place, html_event + GS + path + GS + output_place)
      end
    end

    def set_put_event_listener(input_place, html_event_listener, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("ET" + input_place, html_event_listener + GS + path)
      else
        add("ET" + input_place, html_event_listener + GS + path + GS + output_place)
      end
    end

    def set_patch_event(input_place, html_event, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("Ea" + input_place, html_event + GS + path)
      else
        add("Ea" + input_place, html_event + GS + path + GS + output_place)
      end
    end

    def set_patch_event_listener(input_place, html_event_listener, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("EA" + input_place, html_event_listener + GS + path)
      else
        add("EA" + input_place, html_event_listener + GS + path + GS + output_place)
      end
    end

    def set_delete_event(input_place, html_event, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("El" + input_place, html_event + GS + path)
      else
        add("El" + input_place, html_event + GS + path + GS + output_place)
      end
    end

    def set_delete_event_listener(input_place, html_event_listener, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("EL" + input_place, html_event_listener + GS + path)
      else
        add("EL" + input_place, html_event_listener + GS + path + GS + output_place)
      end
    end

    def set_options_event(input_place, html_event, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("Eo" + input_place, html_event + GS + path)
      else
        add("Eo" + input_place, html_event + GS + path + GS + output_place)
      end
    end

    def set_options_event_listener(input_place, html_event_listener, path_or_output_place = nil, output_place = nil)
      path = path_or_output_place.nil? || path_or_output_place == "#" ? "#" : path_or_output_place
      if output_place.nil?
        add("EO" + input_place, html_event_listener + GS + path)
      else
        add("EO" + input_place, html_event_listener + GS + path + GS + output_place)
      end
    end

    def set_head_event(input_place, html_event, path = nil)
      add("Eh" + input_place, html_event + GS + (!path.nil? && !path.empty? ? path : "#"))
    end

    def set_head_event_listener(input_place, html_event_listener, path = nil)
      add("EH" + input_place, html_event_listener + GS + (!path.nil? && !path.empty? ? path : "#"))
    end

    # IsMultiPart: If this value is true, the data will be sent based on the Form and with the "content" key.
    def set_send_event(input_place, html_event, data, path = nil, method = "POST", is_multi_part = false, content_type = "text/plain", output_place = nil)
      add("En" + input_place, html_event + GS + data.gsub("\n", "$[ln];").gsub("\"", "$[dq];").gsub("'", "$[sq];") + GS + (!path.nil? && !path.empty? ? path : "#") + GS + method + GS + (is_multi_part ? "1" : "0") + GS + content_type + GS + output_place.to_s)
    end

    def set_send_event_listener(input_place, html_event_listener, data, path = nil, method = "POST", is_multi_part = false, content_type = "text/plain", output_place = nil)
      add("EN" + input_place, html_event_listener + GS + data.gsub("\n", "$[ln];") + GS + (!path.nil? && !path.empty? ? path : "#") + GS + method + GS + (is_multi_part ? "1" : "0") + GS + content_type + GS + output_place.to_s)
    end

    def set_comment_event(input_place, html_event, index = nil, output_place = nil)
      add("Eb" + input_place, html_event + GS + index.to_s + GS + output_place.to_s)
    end

    def set_comment_event_listener(input_place, html_event_listener, index = nil, output_place = nil)
      add("EB" + input_place, html_event_listener + GS + index.to_s + GS + output_place.to_s)
    end

    def set_wasm_event(input_place, html_event, wasm_language, wasm_url, method_name, args = nil, output_place = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? "[" + args.join(US) : ""
      end
      add("Ey" + input_place, html_event + GS + wasm_language + GS + wasm_url + GS + method_name + GS + args_join + GS + output_place.to_s)
    end

    def set_wasm_event_listener(input_place, html_event_listener, wasm_language, wasm_url, method_name, args = nil, output_place = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? "[" + args.join(US) : ""
      end
      add("EY" + input_place, html_event_listener + GS + wasm_language + GS + wasm_url + GS + method_name + GS + args_join + GS + output_place.to_s)
    end

    def set_web_socket_event(input_place, html_event, path)
      add("Ew" + input_place, html_event + GS + path)
    end

    def set_web_socket_event_listener(input_place, html_event_listener, path)
      add("EW" + input_place, html_event_listener + GS + path)
    end

    def set_sse_event(input_place, html_event, path, should_reconnect_or_output_place = true, reconnect_try_timeout_or_nil = 3000, output_place = nil)
      if output_place.nil? && should_reconnect_or_output_place.is_a?(TrueClass) || should_reconnect_or_output_place.is_a?(FalseClass)
        add("Ee" + input_place, html_event + GS + path + GS + (should_reconnect_or_output_place ? "1" : "0") + GS + reconnect_try_timeout_or_nil.to_s)
      else
        add("Ee" + input_place, html_event + GS + path + GS + (should_reconnect_or_output_place ? "1" : "0") + GS + reconnect_try_timeout_or_nil.to_s + GS + output_place.to_s)
      end
    end

    def set_sse_event_listener(input_place, html_event_listener, path, should_reconnect_or_output_place = true, reconnect_try_timeout_or_nil = 3000, output_place = nil)
      if output_place.nil? && should_reconnect_or_output_place.is_a?(TrueClass) || should_reconnect_or_output_place.is_a?(FalseClass)
        add("EE" + input_place, html_event_listener + GS + path + GS + (should_reconnect_or_output_place ? "1" : "0") + GS + reconnect_try_timeout_or_nil.to_s)
      else
        add("EE" + input_place, html_event_listener + GS + path + GS + (should_reconnect_or_output_place ? "1" : "0") + GS + reconnect_try_timeout_or_nil.to_s + GS + output_place.to_s)
      end
    end

    def set_front_event(input_place, html_event, module_path, args = nil, output_place = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("Ej" + input_place, html_event + GS + module_path + GS + output_place.to_s + args_join)
    end

    def set_front_event_listener(input_place, html_event_listener, module_path, args = nil, output_place = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("EJ" + input_place, html_event_listener + GS + module_path + GS + output_place.to_s + args_join)
    end

    def set_master_pages_event(input_place, html_event, output_place = nil)
      add("Eu" + input_place, html_event + GS + output_place.to_s)
    end

    def set_master_pages_event_listener(input_place, html_event_listener, output_place = nil)
      add("EU" + input_place, html_event_listener + GS + output_place.to_s)
    end

    def set_prevent_default_event(input_place, html_event)
      add("Ed" + input_place, html_event)
    end

    def set_prevent_default_event_listener(input_place, html_event_listener)
      add("ED" + input_place, html_event_listener)
    end

    def set_stop_propagation_event(input_place, html_event)
      add("Es" + input_place, html_event)
    end

    def set_stop_propagation_event_listener(input_place, html_event_listener)
      add("ES" + input_place, html_event_listener)
    end

    def set_method_event(input_place, html_event, method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("Em" + input_place, html_event + GS + method_name + args_join)
    end

    def set_method_event_listener(input_place, html_event_listener, method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("EM" + input_place, html_event_listener + GS + method_name + args_join)
    end

    def set_module_method_event(input_place, html_event, method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("Ex" + input_place, html_event + GS + method_name + args_join)
    end

    def set_module_method_event_listener(input_place, html_event_listener, method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("EX" + input_place, html_event_listener + GS + method_name + args_join)
    end

    def assign_confirm_event(input_place, html_event, text = "Are you sure you want to proceed?", type = "none", title = "Confirm", ok_text = "OK", cancel_text = "Cancel")
      add("Ef" + input_place, html_event + GS + (text == "Are you sure you want to proceed?" ? "" : text) + GS + (type == "none" ? "" : type) + GS + (title == "Confirm" ? "" : title) + GS + (ok_text == "OK" ? "" : ok_text) + GS + (cancel_text == "Cancel" ? "" : cancel_text))
    end

    def remove_post_event(input_place, html_event)
      add("Rp" + input_place, html_event)
    end

    def remove_post_event_listener(input_place, html_event_listener)
      add("RP" + input_place, html_event_listener)
    end

    def remove_get_event(input_place, html_event)
      add("Rg" + input_place, html_event)
    end

    def remove_get_event_listener(input_place, html_event_listener)
      add("RG" + input_place, html_event_listener)
    end

    def remove_put_event(input_place, html_event)
      add("Rt" + input_place, html_event)
    end

    def remove_put_event_listener(input_place, html_event_listener)
      add("RT" + input_place, html_event_listener)
    end

    def remove_patch_event(input_place, html_event)
      add("Ra" + input_place, html_event)
    end

    def remove_patch_event_listener(input_place, html_event_listener)
      add("RA" + input_place, html_event_listener)
    end

    def remove_delete_event(input_place, html_event)
      add("Rl" + input_place, html_event)
    end

    def remove_delete_event_listener(input_place, html_event_listener)
      add("RL" + input_place, html_event_listener)
    end

    def remove_options_event(input_place, html_event)
      add("Ro" + input_place, html_event)
    end

    def remove_options_event_listener(input_place, html_event_listener)
      add("RO" + input_place, html_event_listener)
    end

    def remove_head_event(input_place, html_event)
      add("Rh" + input_place, html_event)
    end

    def remove_head_event_listener(input_place, html_event_listener)
      add("RH" + input_place, html_event_listener)
    end

    def remove_send_event(input_place, html_event)
      add("Rn" + input_place, html_event)
    end

    def remove_send_event_listener(input_place, html_event_listener)
      add("RN" + input_place, html_event_listener)
    end

    def remove_comment_event(input_place, html_event)
      add("Rb" + input_place, html_event)
    end

    def remove_comment_event_listener(input_place, html_event_listener)
      add("RB" + input_place, html_event_listener)
    end

    def remove_wasm_event(input_place, html_event)
      add("Ry" + input_place, html_event)
    end

    def remove_wasm_event_listener(input_place, html_event_listener)
      add("RY" + input_place, html_event_listener)
    end

    def remove_web_socket_event(input_place, html_event)
      add("Rw" + input_place, html_event)
    end

    def remove_web_socket_event_listener(input_place, html_event_listener)
      add("RW" + input_place, html_event_listener)
    end

    def remove_sse_event(input_place, html_event)
      add("Re" + input_place, html_event)
    end

    def remove_sse_event_listener(input_place, html_event_listener)
      add("RE" + input_place, html_event_listener)
    end

    def remove_front_event(input_place, html_event)
      add("Rj" + input_place, html_event)
    end

    def remove_front_event_listener(input_place, html_event_listener)
      add("RJ" + input_place, html_event_listener)
    end

    def remove_prevent_default_event(input_place, html_event)
      add("Rd" + input_place, html_event)
    end

    def remove_prevent_default_event_listener(input_place, html_event_listener)
      add("RD" + input_place, html_event_listener)
    end

    def remove_master_pages_event(input_place, html_event)
      add("Ru" + input_place, html_event)
    end

    def remove_master_pages_event_listener(input_place, html_event_listener)
      add("RU" + input_place, html_event_listener)
    end

    def remove_stop_propagation_event(input_place, html_event)
      add("Rs" + input_place, html_event)
    end

    def remove_stop_propagation_event_listener(input_place, html_event_listener)
      add("RS" + input_place, html_event_listener)
    end

    def remove_method_event(input_place, html_event, method_name)
      add("Rm" + input_place, html_event + GS + method_name)
    end

    def remove_method_event_listener(input_place, html_event_listener, method_name)
      add("RM" + input_place, html_event_listener + GS + method_name)
    end

    def remove_module_method_event(input_place, html_event, method_name)
      add("Rx" + input_place, html_event + GS + method_name)
    end

    def remove_module_method_event_listener(input_place, html_event_listener, method_name)
      add("RX" + input_place, html_event_listener + GS + method_name)
    end

    def remove_confirm_event(input_place, html_event)
      add("Rf" + input_place, html_event)
    end

    # Custom Event
    # This Method Is Compatible With EventListener And May Not Be Compatible With Events Written As Attributes In Some Browsers.
    # Watch: attribute, style, text, children, value
    # Compare: greater, less, equal, notequal, includes, startswith, endswith, matches, changed, inrange, lengthgreater, lengthless, lengthequal
    # Range: Only Use For Compare With inrange Value. Split By Comma ","
    # Key: Only Use For Watch With attribute And style Value
    def create_custom_dom_event(input_place, event_name, watch, key, compare, value, range, immediate = false, delay = "0")
      add("eC" + input_place, event_name + GS + watch + GS + key + GS + compare + GS + value + GS + range + GS + (immediate ? "1" : "0") + GS + delay.to_s)
    end

    def enable_scroll_bottom_event(enable = true)
      add("eb", enable ? "1" : "0")
    end

    def enable_reached_element_event(input_place, once, enable = true)
      add("er" + input_place, (once ? "1" : "0") + GS + (enable ? "1" : "0"))
    end

    # Module
    def load_module(module_path, methods = nil)
      methods ||= []
      add("Ml", module_path + ((methods.length > 0) ? GS + "[" + methods.join(US) : ""))
    end

    def unload_module(module_path)
      add("Mu", module_path)
    end

    def delete_module_method(method_name)
      add("Md", method_name)
    end

    # Unit Testing
    # InputPlace Is Actual, Expected Is Tag/OutputPlace
    def assert_equal(input_place, tag)
      add("At" + input_place, tag.gsub("\n", "$[ln];"))
    end

    def assert_equal_by_output_place(input_place, output_place)
      add("Ao" + input_place, output_place)
    end

    # Debug
    def create_debugger(pause = false)
      add("Dc", pause ? "1" : "0")
    end

	def try_
	  add("tr")
	end

	def catch_
	  add("ca")
	end

	def comment(text)
	  add("//", text)
	end

    # Service Worker
    # To Use Service Worker, You Need To Add The Elanat Dedicated Module (service-worker.js) On The Client Side
    def service_worker_register(path = nil, scope_path = nil)
      add("wR", path.to_s + GS + scope_path.to_s)
    end

    def service_worker_pre_cache_static(path_list)
      add("wp", path_list.join(GS))
    end

    def service_worker_dynamic_cache(path, seconds = "")
      add("wc", path + (seconds != "" ? GS + seconds.to_s : ""))
    end

    def service_worker_delete_dynamic_cache(path = nil)
      if path.nil?
        add("wd")
      else
        add("wd", path)
      end
    end

    def service_worker_dynamic_cache_ttl_update(path, seconds = "")
      add("wt", path + (seconds != "" ? GS + seconds.to_s : ""))
    end

    # Path: Support Wildcard Automatically And Also Support Regex If Use "re:" Before Pattern
    # Type: Type Is Cache Strategy. cachefirst, networkfirst, cacheonly, networkonly, stalerevalidate (Fast From Cache, Updates Simultaneously From The Network)
    # CacheDynamic: If True, Any Successful Network Response For That Route Will Be Stored In The Dynamic Cache
    def service_worker_route_set(path, type, cache_dynamic = false)
      add("wr", path + GS + type + (cache_dynamic ? GS + "1" : ""))
    end

    def service_worker_route_alias(path, to)
      add("wa", path + GS + to)
    end

    def service_worker_delete_route_alias(path = nil)
      add("wC", path.to_s)
    end

    # Delete All Route And Alias
    def service_worker_delete_route(path = nil)
      if path.nil?
        add("wD")
      else
        add("wD", path)
      end
    end

    # SSE
    def disconnect_sse(path = nil)
      if path.nil?
        add("Ds")
      else
        add("Ds", path)
      end
    end

    def disconnect_all_sse
      add("Ds")
    end

    # State
    def add_state(path = nil, title = nil)
      add("AS", path.to_s + GS + title.to_s)
    end

    def save_state(path = nil, title = nil)
      add("As", path.to_s + GS + title.to_s)
    end

    def load_state(path)
      add("ls", path)
    end

    def delete_state(path = nil)
      if path.nil?
        add("DS")
      elsif path == "*"
        add("DS", "*")
      else
        add("DS", path)
      end
    end

    def delete_all_state
      add("DS", "*")
    end

	def lock_queue(millisecond)
	  add("lq", millisecond.to_s)
	end

    # Cookie
    def set_cookie(key, value, seconds, path = nil)
      add("sC", key + GS + value + GS + seconds.to_s + (!path.nil? && !path.empty? ? GS + path : ""))
    end

    # Save (Session Cache)
    def save_id(input_place, key = ".")
      add("@gi" + input_place, key)
    end

    def save_name(input_place, key = ".")
      add("@gn" + input_place, key)
    end

    def save_value(input_place, key = ".")
      add("@gv" + input_place, key)
    end

    def save_value_length(input_place, key = ".")
      add("@ge" + input_place, key)
    end

    def save_class(input_place, key = ".")
      add("@gc" + input_place, key)
    end

    def save_style(input_place, key = ".")
      add("@gs" + input_place, key)
    end

    def save_title(input_place, key = ".")
      add("@gl" + input_place, key)
    end

    def save_label(input_place, key = ".")
      add("@gA" + input_place, key)
    end

    def save_text(input_place, key = ".")
      add("@gt" + input_place, key)
    end

    def save_outer_text(input_place, key = ".")
      add("@go" + input_place, key)
    end

    def save_text_length(input_place, key = ".")
      add("@gg" + input_place, key)
    end

    def save_attribute(input_place, attribute, key = ".")
      add("@ga" + input_place, key + GS + attribute)
    end

    def save_width(input_place, key = ".")
      add("@gw" + input_place, key)
    end

    def save_height(input_place, key = ".")
      add("@gh" + input_place, key)
    end

    def save_read_only(input_place, key = ".")
      add("@gr" + input_place, key)
    end

    def save_selected_index(input_place, key = ".")
      add("@gx" + input_place, key)
    end

    def save_text_align(input_place, key = ".")
      add("@gT" + input_place, key)
    end

    def save_node_length(input_place, key = ".")
      add("@gL" + input_place, key)
    end

    def save_visible(input_place, key = ".")
      add("@gV" + input_place, key)
    end

    def save_url(url, fetch_script = false, key = ".")
      add("@gu", key + GS + url + (fetch_script ? GS + "1" : ""))
    end

    def save_index(input_place, key = ".")
      add("@gI" + input_place, key)
    end

    def remove_save(cache_key)
      add("rs", cache_key)
    end

    def remove_all_save
      add("rs", "*")
    end

    # Calling the SetSave Method Causes Action Control Requests Triggered by Events Using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send Event, to be Temporarily Saved on the Active Page, so the Request will not be Sent to the Server Again.
    def set_save
      add("cs", "*")
    end

    def add_save_value(cache_key, value)
      add("SA", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def insert_save_value(cache_key, value)
      add("SI", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def append_save_value(cache_key, value)
      add("SP", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def replace_save_value(cache_key, search_value, value)
      add("SR", cache_key + GS + value.gsub("\n", "$[ln];") + GS + search_value.gsub("\n", "$[ln];"))
    end

	# Is Regex Replace
	def set_format_save_value(cache_key, regex, replacement)
	  add("SF", cache_key + GS + regex + GS + replacement.gsub("\n", "$[ln];"))
	end

	# Operator: +, -, *, /, %, //, **
	def set_arithmetic_save_value(cache_key, operator, value)
	  add("SM", cache_key + GS + operator + GS + value.to_s)
	end

	# Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	def set_text_operation_save_value(cache_key, operation, value1, value2)
	  add("ST", cache_key + GS + operation + GS + value1.to_s + GS + value2.to_s)
	end

    # Cache
    def cache_id(input_place, key = ".")
      add("@ci" + input_place, key)
    end

    def cache_name(input_place, key = ".")
      add("@cn" + input_place, key)
    end

    def cache_value(input_place, key = ".")
      add("@cv" + input_place, key)
    end

    def cache_value_length(input_place, key = ".")
      add("@ce" + input_place, key)
    end

    def cache_class(input_place, key = ".")
      add("@cc" + input_place, key)
    end

    def cache_style(input_place, key = ".")
      add("@cs" + input_place, key)
    end

    def cache_title(input_place, key = ".")
      add("@cl" + input_place, key)
    end

    def cache_label(input_place, key = ".")
      add("@cA" + input_place, key)
    end

    def cache_text(input_place, key = ".")
      add("@ct" + input_place, key)
    end

    def cache_outer_text(input_place, key = ".")
      add("@co" + input_place, key)
    end

    def cache_text_length(input_place, key = ".")
      add("@cg" + input_place, key)
    end

    def cache_attribute(input_place, attribute, key = ".")
      add("@ca" + input_place, key + GS + attribute)
    end

    def cache_width(input_place, key = ".")
      add("@cw" + input_place, key)
    end

    def cache_height(input_place, key = ".")
      add("@ch" + input_place, key)
    end

    def cache_read_only(input_place, key = ".")
      add("@cr" + input_place, key)
    end

    def cache_selected_index(input_place, key = ".")
      add("@cx" + input_place, key)
    end

    def cache_text_align(input_place, key = ".")
      add("@cT" + input_place, key)
    end

    def cache_node_length(input_place, key = ".")
      add("@cL" + input_place, key)
    end

    def cache_visible(input_place, key = ".")
      add("@cV" + input_place, key)
    end

    def cache_url(url, fetch_script = false, key = ".")
      add("@cu", key + GS + url + (fetch_script ? GS + "1" : ""))
    end

    def cache_index(input_place, key = ".")
      add("@cI" + input_place, key)
    end

    def remove_cache(cache_key)
      add("rd", cache_key)
    end

    def remove_all_cache
      add("rd", "*")
    end

    # Calling the SetCache Method Causes Action Control Requests Triggered by events using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send event, to be Cached, so the Request will not be Sent to the Server Again.
    def set_cache(second = nil)
      if second.nil?
        add("cd", "*")
      else
        add("cd", second.to_s)
      end
    end

    def add_cache_value(cache_key, value)
      add("CA", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def insert_cache_value(cache_key, value)
      add("CI", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def append_cache_value(cache_key, value)
      add("CP", cache_key + GS + value.gsub("\n", "$[ln];"))
    end

    def replace_cache_value(cache_key, search_value, value)
      add("CR", cache_key + GS + value.gsub("\n", "$[ln];") + GS + search_value.gsub("\n", "$[ln];"))
    end

	# Is Regex Replace
	def set_format_cache_value(cache_key, regex, replacement)
	  add("CF", cache_key + GS + regex + GS + replacement.gsub("\n", "$[ln];"))
	end

	# Operator: +, -, *, /, %, //, **
	def set_arithmetic_cache_value(cache_key, operator, value)
	  add("CM", cache_key + GS + operator + GS + value.to_s)
	end

	# Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	def set_text_operation_cache_value(cache_key, operation, value1, value2)
	  add("CT", cache_key + GS + operation + GS + value1.to_s + GS + value2.to_s)
	end

    # Call
    def load_url(input_place, url)
      add("lu" + input_place, url)
    end

    def run_action_controls(action_controls, without_web_forms_section = true, index = nil, use_current_event = true)
      add("lA", (use_current_event ? "1" : "0") + GS + (without_web_forms_section ? "1" : "0") + GS + index.to_s + GS + action_controls)
    end

    def call_script(script_text)
      add("_", script_text.gsub("\n", "$[ln];"))
    end

    def call_method(method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("lm", method_name + args_join)
    end

    def call_module_method(method_name, args = nil)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("lM", method_name + args_join)
    end

    def call_post_back(form_input_place, output_place = nil)
      add("Lp", "1" + GS + form_input_place + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_comment_back(index, input_place = nil, use_current_event = true)
      add("LC", (use_current_event ? "1" : "0") + GS + index.to_s + GS + input_place.to_s)
    end

    def call_wasm_back(wasm_language, wasm_url, method_name, args = nil, output_place = nil, use_current_event = true)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? "[" + args.join(US) : ""
      end
      add("Ly", (use_current_event ? "1" : "0") + GS + wasm_language + GS + wasm_url + GS + method_name + GS + args_join + GS + output_place.to_s)
    end

    def call_web_socket_back(path, use_current_event = true)
      add("Lw", (use_current_event ? "1" : "0") + GS + path)
    end

    def call_sse_back(path, output_place = nil, use_current_event = true, should_reconnect = true, reconnect_try_timeout = "3000")
      add("Ls", (use_current_event ? "1" : "0") + GS + path + GS + (should_reconnect ? "1" : "0") + GS + reconnect_try_timeout.to_s + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_front(module_path, args = nil, output_place = nil, use_current_event = true)
      args_join = ""
      if !args.nil? && args.is_a?(Array)
        args_join = (args.length > 0) ? GS + "[" + args.join(US) : ""
      end
      add("Lj", (use_current_event ? "1" : "0") + GS + module_path + GS + output_place.to_s + args_join)
    end

    def call_get_back(path, output_place = nil, use_current_event = true)
      add("Lg", (use_current_event ? "1" : "0") + GS + path + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_put_back(path, output_place = nil, use_current_event = true)
      add("Lt", (use_current_event ? "1" : "0") + GS + path + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_patch_back(path, output_place = nil, use_current_event = true)
      add("LP", (use_current_event ? "1" : "0") + GS + path + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_delete_back(path, output_place = nil, use_current_event = true)
      add("Ld", (use_current_event ? "1" : "0") + GS + path + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_head_back(path, use_current_event = true)
      add("Lh", (use_current_event ? "1" : "0") + GS + path)
    end

    def call_options_back(path, output_place = nil, use_current_event = true)
      add("Lo", (use_current_event ? "1" : "0") + GS + path + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    def call_send_back(path, method, is_multi_part, content_type, data, output_place = nil, use_current_event = true)
      add("LS", (use_current_event ? "1" : "0") + GS + path + GS + method + GS + (is_multi_part ? "1" : "0") + GS + content_type + GS + data.gsub("\n", "$[ln];") + (!output_place.nil? && !output_place.empty? ? GS + output_place : ""))
    end

    # Update
    def increase(input_place, value)
      add("gt" + input_place, "i" + GS + value.to_s)
    end

    def decrease(input_place, value)
      add("gt" + input_place, "i" + GS + (value * -1).to_s)
    end

    # If You Don't Use Deep Mode, any Tags Inside the Current Tag Will Simply Be Treated as Strings. Deep Mode Does not Remove Inner Elements.
    def replace(input_place, value, new_value, also_start_tag = false, deep = true)
      add("gt" + input_place, "r" + GS + value + GS + new_value + GS + (also_start_tag ? "1" : "0") + GS + (deep ? "1" : "0"))
    end

    # HTML Converts Attribute Names to Lowercase, so they Need to Be Written in Lowercase.
    def replace_start_tag(input_place, value, new_value)
      add("gt" + input_place, "s" + GS + value + GS + new_value)
    end

    # Pre Runner
    def assign_delay(milli_second, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      new_name = ":" + milli_second.to_s + ")" + parts[0]
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    def assign_delay_change(milli_second, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      current_name = parts[0]

      if current_name.start_with?(":") && current_name.include?(")")
        closing_bracket = current_name.index(")")
        current_name = current_name[(closing_bracket + 1)..-1]
      end

      new_name = ":" + milli_second.to_s + ")" + current_name
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    def assign_interval(milli_second, id = nil, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      new_name = "(" + milli_second.to_s + (!id.nil? && !id.empty? ? "|" + id : "") + ")" + parts[0]
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    def assign_interval_change(milli_second, id = nil, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      current_name = parts[0]

      if current_name.start_with?("(") && current_name.include?(")")
        closing_bracket = current_name.index(")")
        current_name = current_name[(closing_bracket + 1)..-1]
      end

      new_name = "(" + milli_second.to_s + (!id.nil? && !id.empty? ? "|" + id : "") + ")" + current_name
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    def delete_interval(id)
      add("Di", id)
    end

    def assign_repeat(count, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      new_name = "," + count.to_s + ")" + parts[0]
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    def assign_repeat_change(count, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      current_name = parts[0]

      if current_name.start_with?(",") && current_name.include?(")")
        closing_bracket = current_name.index(")")
        current_name = current_name[(closing_bracket + 1)..-1]
      end

      new_name = "," + count.to_s + ")" + current_name
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    # Index
    def start_index(name = "")
      add("#", name)
    end

    # This Index Is Automatically Run After Changing The Browser History (Back And Forward Buttons)
    def start_state
      start_index("$")
    end

    def go_to(line, repeat = 1)
      if line.is_a?(Integer)
        add("&", line.to_s + GS + repeat.to_s)
      elsif repeat.is_a?(Integer) && !line.is_a?(Integer)
        add("&", "#" + line + GS + repeat.to_s)
      else
        add("&", line.to_s + GS + repeat.to_s)
      end
    end
    
    # Start
    def start_transient_dom(input_place)
      add("td", input_place)
    end

    def end_transient_dom
      add("td", ";")
    end

    # Message
    # Type: warning, problem, help, success, none
    def alert(text, type = "none", title = "Alert", ok_text = "OK")
      add("Al", text + GS + (type == "none" ? "" : type) + GS + (title == "Alert" ? "" : title) + GS + (ok_text == "OK" ? "" : ok_text))
    end

    def message(text, type_or_duration = "none", duration = "0")
      if type_or_duration.is_a?(Integer)
        add("me", text + GS + "" + GS + type_or_duration.to_s)
      elsif duration.is_a?(Integer)
        add("me", text + GS + (type_or_duration == "none" ? "" : type_or_duration) + GS + duration.to_s)
      else
        add("me", text + GS + (type_or_duration == "none" ? "" : type_or_duration) + GS + (duration == "0" ? "" : duration))
      end
    end

    # Type: log, info, warn, error, debug, trace, group, groupend, table
    def console_message(text, type = "log")
      add("mc", text.gsub("\n", "$[ln];") + (type == "log" ? "" : GS + type))
    end

    def console_message_assert(text, condition)
      add("ma", text.gsub("\n", "$[ln];") + GS + condition)
    end

    # Enable
    # Calling The EnableWebSocket Or EnableWebSocketOnce Or AddWebSocket Methods Will Cause Any Subsequent Requests (Under WebForms Core Technology) To Operate Under The WebSocket Protocol.
    def enable_web_socket(enable = true)
      add("ew", enable ? "1" : "0")
    end

    def enable_web_socket_once
      add("ew", "$")
    end

    def add_web_socket(path)
      add("aw" + path)
    end

    # Disconnected WebSocket
    def delete_web_socket(path)
      add("dw" + path)
    end

    # Use
    # InputPlace Using Only For form Element
    def use_web_socket(input_place)
      add("uw" + input_place)
    end

    def use_only_change_update(input_place)
      add("uo" + input_place)
    end

    # Condition And Loop
    # Condition And Loop Supports Brackets and Then
    # Type: warning, problem, help, success, none
    # Interval: Value 0 is Await (if is not True, all Next Action Controls Waiting for it), Value -1 is Sync Check Once (is Support Bracket or Next Action Control), Value > 0 is Async and is Wait Based on Time Repetition Until it Becomes True (Is Support Bracket or Next Action Control, but is not Support Else).
    # Nested Conditions and Nested Loops are Possible.
    def confirm_is_true_accept(text = "Are you sure you want to proceed?", type = "none", title = "Confirm", ok_text = "OK", cancel_text = "Cancel", interval = 100)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "ct", (text == "Are you sure you want to proceed?" ? "" : text) + GS + (type == "none" ? "" : type) + GS + (title == "Confirm" ? "" : title) + GS + (ok_text == "OK" ? "" : ok_text) + GS + (cancel_text == "Cancel" ? "" : cancel_text))
      self
    end

    def confirm_is_false_accept(text = "Are you sure you want to proceed?", type = "none", title = "Confirm", ok_text = "OK", cancel_text = "Cancel", interval = 100)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "cf", (text == "Are you sure you want to proceed?" ? "" : text) + GS + (type == "none" ? "" : type) + GS + (title == "Confirm" ? "" : title) + GS + (ok_text == "OK" ? "" : ok_text) + GS + (cancel_text == "Cancel" ? "" : cancel_text))
      self
    end

    def is_greater_than(first_value, second_value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "gt", first_value + GS + second_value)
      self
    end

    def is_less_than(first_value, second_value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "lt", first_value + GS + second_value)
      self
    end

    def is_equal_to(first_value, second_value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "et", first_value + GS + second_value)
      self
    end

    def is_not_equal_to(first_value, second_value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "Nt", first_value + GS + second_value)
      self
    end

    def exist(value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "ex", value)
      self
    end

    def not_exist(value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "nx", value)
      self
    end

    def is_true(value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "tr", value)
      self
    end

    def is_false(value, interval = 1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "fa", value)
      self
    end

    def is_match_media(value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "mm", value)
      self
    end

    def is_not_match_media(value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "nm", value)
      self
    end

    def include(text, value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "In", value + GS + text)
      self
    end

    def not_include(text, value, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "Nn", value + GS + text)
      self
    end

    def element_exists(input_place, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "eE", input_place)
      self
    end

    def element_not_exists(input_place, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "nE", input_place)
      self
    end

    def is_regex_match(value, pattern, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "re", value + GS + pattern)
      self
    end

    def is_regex_not_match(value, pattern, interval = -1)
      add(((interval >= 0) ? "{(" + interval.to_s + ")" : "{") + "rn", value + GS + pattern)
      self
    end

    # In: Everything Becomes A JSON List.
    # Key: Creates A Temporary Data In The Browser IndexedDB.
    # Key + "i" Creates A Temporary Data To Maintain The Loop Counter In The Browser IndexedDB.
    def for_each(path, in_val, key = ".")
      add("{fe", path + GS + in_val + GS + key)
      self
    end

    def break_
      add(";")
    end

    def else_
      add("}e")
      self
    end

    def start_bracket
      add("{")
    end

    def end_bracket
      add("}")
    end

	# High-Level Methods
    # Used Then In Condition And Loop Methods
    def then_(new_form = nil, &block)
      if block_given?
        new_form = WebForms.new
        yield new_form
      end
      
      return self if new_form.nil?

      data = new_form.get_web_forms_data
      
      if !data.nil? && !data.empty?
        if data.include?("\n")
          new_form.add_to_up("{")
          new_form.add("}")
        end
      end
      
      append_form(new_form)
      self
    end

	def repeat(new_form = nil, repeat_count = 1, index = nil, &block)
	  if block_given?
		temp_form = WebForms.new
		yield temp_form
		return repeat(temp_form, repeat_count, index)
	  end

	  return self if new_form.nil?

	  if index.nil?
		body_data = new_form.get_web_forms_data
		return self if body_data.nil? || body_data.empty?

		start_line = body_data.split("\n", -1).length * -1
		append_form(new_form)
		go_to(start_line, repeat_count - 1)
	  else
		go_to(index)
		start_index(index)

		body_data = new_form.get_web_forms_data
		return self if body_data.nil? || body_data.empty?

		append_form(new_form)

		if index.empty?
		  index_number = -1
		  get_web_forms_data.split("\n").each do |x|
			index_number += 1 if x.start_with?("#")
		  end
		  go_to(index_number.to_s, repeat_count - 1)
		else
		  go_to(index, repeat_count - 1)
		end
	  end

	  self
	end

	def isole(new_form = nil, input_place = nil, &block)
	  if block_given?
		temp_form = WebForms.new
		yield temp_form
		return isole(temp_form, input_place)
	  end

	  return self if new_form.nil?

	  body_data = new_form.get_web_forms_data
	  return self if body_data.nil? || body_data.empty?

	  form = WebForms.new
	  form.start_transient_dom(input_place)

	  append_form(form)
	  new_form.end_transient_dom
	  append_form(new_form)

	  self
	end

    # The Render Method is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
    # This Method Utilizes the Transient DOM; Therefore, When Selecting InputPlaces, You Must Consider the Root of the Selected InputPlace Within the Method.
	def render(new_form = nil, input_place = nil, key = "", permanent = false, &block)
	  if block_given?
		temp_form = WebForms.new
		yield temp_form
		return render(temp_form, input_place, key, permanent)
	  end

	  return self if new_form.nil?

	  body_data = new_form.get_web_forms_data
	  return self if body_data.nil? || body_data.empty?

	  key = input_place if key.nil? || key.empty?

	  form = WebForms.new

	  form.exist(permanent ? Fetch.cache(key) : Fetch.save(key))
		form.rollback(input_place, key, permanent)
	  form.else_
		form.snapshot(input_place, key, permanent)

	  form.start_transient_dom(input_place)

	  append_form(form)
	  new_form.end_transient_dom
	  append_form(new_form)

	  self
	end

    # Async
    # It Supports Brackets and Then
    def async_
      add("{(a)")
      self
    end

    def delay(milli_second)
      add("De", milli_second.to_s)
    end

    # Option
    def change_option(name, value)
      add("co", name + GS + value)
    end

    def reset_option(name = nil)
      if name.nil?
        add("ro")
      else
        add("ro", name)
      end
    end

    # Format Storage
    def create_format_storage(key, data)
      add(".C", key + GS + data)
    end

    def delete_format_storage(key)
      add(".D", key)
    end

    def add_json(key, path, value)
      add(".a", key + GS + "j" + GS + value + GS + path)
    end

    # Name: For Support Attribute, Set Double At Sign (@@) Before Name.
    def add_xml(key, path, name, value = nil)
      add(".a", key + GS + "x" + GS + name + GS + value.to_s + GS + path)
    end

    def add_ini(key, path, value, is_ini_like = false)
      add(".a", key + GS + "i" + GS + (is_ini_like ? "1" : "0") + GS + value + GS + path)
    end

    def add_text_line(key, line, text)
      add(".a", key + GS + "t" + GS + text + GS + line.to_s)
    end

    def add_variable(key, value)
      add(".a", key + GS + "v" + GS + value)
    end

    def update_json(key, path, value)
      add(".u", key + GS + "j" + GS + value + GS + path)
    end

    def update_xml(key, path, value)
      add(".u", key + GS + "x" + GS + value + GS + path)
    end

    def update_ini(key, path, value, is_ini_like = false)
      add(".u", key + GS + "i" + GS + (is_ini_like ? "1" : "0") + GS + value + GS + path)
    end

    # Note: C# original has a typo 'UpdateTexLine', mapped to 'update_text_line' for Ruby standards while preserving exact logic.
    def update_text_line(key, line, text)
      add(".u", key + GS + "t" + GS + text + GS + line.to_s)
    end

    def update_variable(key, value)
      add(".u", key + GS + "v" + GS + value)
    end

    def increase_variable(key, value)
      add(".i", key + GS + "v" + GS + value.to_s)
    end

    def decrease_variable(key, value)
      increase_variable(key, value * -1)
    end

    def delete_json(key, path)
      add(".d", key + GS + "j" + GS + path)
    end

    def delete_xml(key, path)
      add(".d", key + GS + "x" + GS + path)
    end

    def delete_ini(key, path, is_ini_like = false)
      add(".d", key + GS + "i" + GS + is_ini_like.to_s + GS + path)
    end

    def delete_text_line(key, line)
      add(".d", key + GS + "t" + GS + line.to_s)
    end

    def delete_variable(key)
      add(".d", key + GS + "v")
    end

    # Template Engine
    # Pattern Example: {{value}}, ((value)), *value*, $value;
    def bind_json_to_template(input_place, json_text, path, pattern, also_start_tag = true)
      add("Tj" + input_place, json_text + GS + path + GS + pattern + GS + (also_start_tag ? "1" : "0"))
    end

    # Because XML Elements Are Lowercased, Placeholders Must Use Lowercase Names.
    def bind_xml_to_template(input_place, xml_text, path, pattern, also_start_tag = true)
      add("Tx" + input_place, xml_text + GS + path + GS + pattern + GS + (also_start_tag ? "1" : "0"))
    end

    def bind_ini_to_template(input_place, ini_text, path, pattern, also_start_tag = true)
      add("Ti" + input_place, ini_text + GS + path + GS + pattern + GS + (also_start_tag ? "1" : "0"))
    end

    # Inject
    # Need Add @: to First of String
    def inject(value)
      "$[" + value + "];"
    end

    # Action Control
    def replace_action_control(search_value, value, adding_to_up = false)
      if adding_to_up
        add_to_up("rE", search_value + GS + value)
      else
        add("rE", search_value + GS + value)
      end
    end
    
    def assign_replace(search_value, value, index = -1)
      current_line = get_line_by_index(index)
      return if current_line.nil? || current_line.empty?

      parts = current_line.split('=', 2)
      new_name = ";" + search_value + GS + value + GS + parts[0]
      new_value = parts.length > 1 ? parts[1] : ""

      update_line_by_index(index, new_name, new_value)
    end

    # Hash And Checksum
    def set_hash
      add("SH")
    end

    def set_checksum
      add("CS")
    end

    def checksum_calculation(text)
      sum = 0
      mod = 65536
      shift = 5

      text.each_char do |c|
        sum = (((sum << shift) | (sum >> (16 - shift))) ^ c.ord) % mod
      end

      sum.to_s
    end

    def get_checksum
      checksum_calculation(get_web_forms_data())
    end

    # Get
    def get_forms_action_data
      return "" if @web_forms_data.length == 0
      @web_forms_data
    end

    def response
      "[web-forms]\n" + get_forms_action_data()
    end

    def get_forms_action_data_line_break
      return "" if @web_forms_data.length == 0

      data = @web_forms_data
      processed_data = data.gsub("\"", "$[dq];")
      processed_data.gsub("\n", "$[sln];")
    end

    # Export
    def export_to_html_comment(add_line = false)
      response_str = response().gsub("--", "$[dd];")
      if response_str[-1] == '-'
        response_str = response_str[0...-1] + "$[da];"
      end

      (add_line ? "\n" : "") + "<!--" + response_str + "-->"
    end

    # Using it for SSE Response
    def export_to_line_break(src = nil)
      "[web-forms]$[sln];" + get_forms_action_data_line_break()
    end

    def get_web_forms_data
      @web_forms_data
    end

    def append_form(form)
      return if form.nil?

      other_data = form.get_web_forms_data
      if !other_data.nil? && !other_data.empty?
        if @web_forms_data.length > 0
          @web_forms_data << "\n"
        end
        @web_forms_data << other_data
      end
    end

    def clean
      @web_forms_data.clear
    end
  end

  class Security
    def safe_value(value)
      return value if value.length < 1

      value = "@" + value if value[0] == '@'

      value = value
        .gsub("\n", "$[ln];")
        .gsub(",@", "$[co];@")
        .gsub(28.chr, "\0")
        .gsub(29.chr, "\0")
        .gsub(30.chr, "\0")
        .gsub(31.chr, "\0")

      value
    end
  end

  # WebForms Place Criteria (WPC) DSL
  class InputPlace
    def self.document = ","
    def self.window = "`"
    # When Calling TransientDOM, Using Root will Result in the Selection of the Transient Tag.
    def self.root = "~"
    def self.html = "."
    def self.head = "^"
    def self.screen_orientation = "%"
    def self.all = "*"
    def self.parent = "/"
    def self.current = "$"
    def self.target = "!"
    def self.upper = "-"

    def self.id(id_val)
      id_val
    end

    def self.name(name_val, index = nil)
      if index.nil?
        '(' + name_val + ')'
      else
        '(' + name_val + ')' + index.to_s
      end
    end

    def self.all_names(name_val)
      "(" + name_val + ")*"
    end

    def self.tag(tag_val, index = nil)
      if index.nil?
        '<' + tag_val + '>'
      else
        '<' + tag_val + '>' + index.to_s
      end
    end

    def self.all_tags(tag_val)
      "<" + tag_val + ">*"
    end

    def self.child(index = nil)
      if index.nil?
        "<>"
      else
        "<>" + index.to_s
      end
    end

    def self.all_child
      "<>*"
    end

    def self.class_name(class_val, index = nil)
      if index.nil?
        '{' + class_val + '}'
      else
        '{' + class_val + '}' + index.to_s
      end
    end

    def self.all_classes(class_val)
      "{" + class_val + "}*"
    end

    def self.attribute(name_val, value_or_index = nil, operator_or_nil = nil, index = nil)
      if value_or_index.nil?
        '"' + name_val + '"'
      elsif index.nil? && (operator_or_nil.nil? || operator_or_nil == "\0")
        '"' + name_val + '"' + value_or_index.to_s
      else
        op = operator_or_nil.nil? || operator_or_nil == "\0" ? "" : operator_or_nil.to_s
        if index.nil?
          '"' + name_val + op + "'" + value_or_index + '"'
        else
          '"' + name_val + op + "'" + value_or_index + '"' + index.to_s
        end
      end
    end

    def self.all_attributes(name_val, value = nil, operator = "\0")
      if value.nil?
        "\"" + name_val + "\"*"
      else
        op = operator == "\0" ? "" : operator.to_s
        "\"" + name_val + op + "'" + value + "\"*"
      end
    end

    def self.query(query_val)
      "*" + query_val.gsub("=", "$[eq];").gsub("|", "$[vb];").gsub("?", "$[qu];")
    end

    def self.query_all(query_val)
      "[" + query_val.gsub("=", "$[eq];").gsub("|", "$[vb];").gsub("?", "$[qu];")
    end
  end

  class OutputPlace < InputPlace
  end

  # Do not Add any Data Before or After it
  class Fetch
    RS = 30.chr
    US = 31.chr

    # Method
    def self.random(max_value, min_value = nil)
      if min_value.nil?
        "@mr" + max_value.to_s
      else
        "@mr" + max_value.to_s + RS + min_value.to_s
      end
    end

    def self.space_to_char(text, character = "-")
      "@sc" + character + RS + text
    end

    def self.encode_uri(text)
      "@ue" + text
    end

    def self.decode_uri(text)
      "@ud" + text
    end

    def self.method(method_name, args = nil)
      return_value = "@cm" + method_name
      if !args.nil? && args.is_a?(Array)
        return_value += (args.length > 0) ? RS + args.join(US) : ""
      end
      return_value
    end

    def self.module_method(method_name, args = nil)
      return_value = "@cM" + method_name
      if !args.nil? && args.is_a?(Array)
        return_value += (args.length > 0) ? RS + args.join(US) : ""
      end
      return_value
    end

    # MethodName: The Method Name May Need to Include the Class Name, Separated by a Period. Example: MyClassName.MyMethodName
    def self.wasm_method(wasm_language, wasm_url, method_name, args = nil)
      return_value = "@wA" + wasm_language + RS + wasm_url + RS + method_name
      if !args.nil? && args.is_a?(Array)
        return_value += (args.length > 0) ? RS + args.join(US) : ""
      end
      return_value
    end

    def self.script(script_text)
      "@_" + script_text.gsub("\n", "$[ln];")
    end

    def self.load_url(url, fetch_script = false)
      "@lu" + url + (fetch_script ? RS + "1" : "")
    end

    def self.load_html(url, fetch_input_place = "", fetch_script = false)
      "@lh" + url + RS + (fetch_script ? "1" : "0") + (!fetch_input_place.nil? && !fetch_input_place.empty? ? RS + fetch_input_place : "")
    end

    def self.load_line(url, line)
      "@ll" + url + RS + line.to_s
    end

    def self.load_ini(url, name, is_ini_like = false)
      "@li" + url + RS + name + (is_ini_like ? RS + "1" : "")
    end

    # Name: Name Or Nested Paths. Is Supprt Index (Student[8].Name). Nested Paths Index Starts At 0
    def self.load_json(url, name)
      "@lj" + url + RS + name
    end

    # Name: Name Or XPath; XPath Index Starts At 1
    def self.load_xml(url, name)
      "@lx" + url + RS + name
    end

    # MethodName: It's Check Function Or Variable
    def self.has_method(method_name)
      "@hm" + method_name
    end

    def self.has_module_method(method_name)
      "@hM" + method_name
    end

    # This Method Return True Or False If Key Pressed
    # Modifier: Alt, AltGraph, Control, Meta, Shift, CapsLock, NumLock, ScrollLock
    def self.get_modifier_state(modifier)
      "@ms" + modifier
    end

    # Math
    def self.math(method_name, args = nil)
      return_value = "@M#" + method_name
      if !args.nil? && args.is_a?(Array)
        return_value += (args.length > 0) ? RS + args.join(US) : ""
      end
      return_value
    end

	# Date
	def self.date_year = "@dy"
	# Month In JavaScript Is Start From Index 0, Month In WebForms Core Is Start From Index 1
	def self.date_month = "@dm"
	def self.date_day = "@dd"
	def self.date_date = "@dD"
	def self.date_hours = "@dh"
	def self.date_minutes = "@di"
	def self.date_seconds = "@ds"
	def self.date_milliseconds = "@dl"

	# String
	def self.space = "@sp"
	def self.at_sign = "@sa"

    # Tag
    def self.get_id(input_place)
      "@$i" + input_place
    end

    def self.get_name(input_place)
      "@$n" + input_place
    end

    def self.get_value(input_place)
      "@$v" + input_place
    end

    def self.get_value_length(input_place)
      "@$e" + input_place
    end

    def self.get_class(input_place)
      "@$c" + input_place
    end

    def self.get_style(input_place)
      "@$s" + input_place
    end

    def self.get_title(input_place)
      "@$l" + input_place
    end

    def self.get_label(input_place)
      "@$A" + input_place
    end

    def self.get_text(input_place)
      "@$t" + input_place
    end

    def self.get_outer_text(input_place)
      "@$o" + input_place
    end

    def self.get_text_length(input_place)
      "@$g" + input_place
    end

    def self.get_attribute(input_place, attribute)
      "@$a" + input_place + RS + attribute
    end

    def self.get_width(input_place)
      "@$w" + input_place
    end

    def self.get_height(input_place)
      "@$h" + input_place
    end

    def self.get_is_read_only(input_place)
      "@$r" + input_place
    end

    def self.get_selected_index(input_place)
      "@$x" + input_place
    end

    def self.get_index(input_place)
      "@$I" + input_place
    end

    def self.get_text_align(input_place)
      "@$T" + input_place
    end

    def self.get_node_length(input_place)
      "@$L" + input_place
    end

    def self.get_is_visible(input_place)
      "@$V" + input_place
    end

	def self.get_tag_hash(input_place)
	  "@$H" + input_place
	end

    # Save and Cache
    def self.has_hash(hash_val)
      "@HH" + hash_val
    end

    def self.cookie(key)
      "@co" + key
    end

    def self.save(key = ".", replace_value = nil)
      if replace_value.nil?
        "@cs" + key
      else
        "@cs" + key + RS + replace_value
      end
    end

    def self.save_then_remove(key)
      "@cl" + key
    end

    def self.save_length(key = ".")
      "@cg" + key
    end

    def self.cache(key = ".", replace_value = nil)
      if replace_value.nil?
        "@cd" + key
      else
        "@cd" + key + RS + replace_value
      end
    end

    def self.cache_then_remove(key)
      "@ct" + key
    end

    def self.cache_length(key = ".")
      "@cG" + key
    end

    def self.save_line(key = ".", line = 0)
      "@lL" + key + "[" + line.to_s
    end

    def self.save_line_consume(key = ".")
      "@lL" + key
    end

    # INIKey: Only Direct Key is Supported
    def self.save_ini(key, ini_key)
      "@lI" + key + "[" + ini_key
    end

    def self.cache_line(key = ".", line = 0)
      "@dL" + key + "[" + line.to_s
    end

    def self.cache_line_consume(key = ".")
      "@dL" + key
    end

    # INIKey: Only Direct Key is Supported
    def self.cache_ini(key, ini_key)
      "@dI" + key + "[" + ini_key
    end

    # Format Storage
    def self.format_store(key)
      "@fr" + key
    end

    def self.format_store_by_xml_query(key, xpath)
      "@fx" + key + RS + xpath
    end

    def self.format_store_by_json_query(key, query)
      "@fj" + key + RS + query
    end

    def self.format_store_by_ini(key, name)
      "@fi" + key + RS + name
    end

    def self.format_store_by_text(key, line)
      "@ft" + key + RS + line.to_s
    end

    def self.format_store_by_variable(key)
      "@fv" + key
    end

    # State
    def self.has_state(path)
      "@hs" + path
    end

    # SSE
    def self.sse_is_connected(path)
      "@Sc" + path
    end

    # WebSockets
    def self.web_sockets_is_connected(path = "")
      "@Wc" + path
    end

    # Document
    def self.tab_is_active = "@da"

    # Window
    def self.href = "@wf"
    PathName = "@wP"
    
    def self.query(name = "*")
      "@wq" + name
    end
    
	def self.hash = "@wh"
	def self.host = "@wH"
	def self.host_name = "@wn"
	def self.port = "@wT"
	def self.origin = "@wo"
	def self.get_selection = "@ws"
	def self.scroll_x = "@wx"
	def self.scroll_y = "@wy"
    
    def self.segment(index)
      "@wS" + index.to_s
    end
    
    # It Only Works when the String Starts with the Tilde Character (~). The Path is Also Separated by the Slash Character (/). #~/Segment1/Segment2/Segment3
    def self.hash_segment(index)
      "@wt" + index.to_s
    end

    # Navigator
    def self.clipboard_text = "@nC"
    def self.geo_latitude = "@nW"
    def self.geo_longitude = "@nO"
    def self.language = "@nL"
    def self.is_on_line = "@no"
    def self.user_agent = "@na"

    # Screen
    def self.screen_width = "@sw"
    def self.screen_height = "@sh"
    def self.screen_orientation_type = "@so"
    def self.screen_orientation_angle = "@sr"

    # Performance
    def self.time_origin = "@pt"
    def self.performance_now = "@pn"

    # Event
    def self.event = "@EV"
    def self.event_serialize = "@Es"
    def self.event_key = "@ek"
    def self.event_which = "@ew"
    def self.event_client_x = "@ex"
    def self.event_client_y = "@ey"
    def self.event_page_x = "@eX"
    def self.event_page_y = "@eY"
    def self.event_offset_x = "@Ex"
    def self.event_offset_y = "@Ey"
    def self.event_delta_y = "@ed"
  end
  
  class WasmLanguage
    def self.c = "c"
    def self.cpp = "c"
    def self.rust = "rust"
    def self.c_sharp = "csharp"
    def self.go = "go"
    def self.java = "java"
    def self.assembly_script = "as"
  end

  class HtmlEvent
    def self.on_abort = "onabort"
    def self.on_after_print = "onafterprint"
    def self.on_before_print = "onbeforeprint"
    def self.on_before_unload = "onbeforeunload"
    def self.on_blur = "onblur"
    def self.on_can_play = "oncanplay"
    def self.on_can_play_through = "oncanplaythrough"
    def self.on_change = "onchange"
    def self.on_click = "onclick"
    def self.on_copy = "oncopy"
    def self.on_cut = "oncut"
    def self.on_double_click = "ondblclick"
    def self.on_drag = "ondrag"
    def self.on_drag_end = "ondragend"
    def self.on_drag_enter = "ondragenter"
    def self.on_drag_leave = "ondragleave"
    def self.on_drag_over = "ondragover"
    def self.on_drag_start = "ondragstart"
    def self.on_drop = "ondrop"
    def self.on_duration_change = "ondurationchange"
    def self.on_ended = "onended"
    def self.on_error = "onerror"
    def self.on_focus = "onfocus"
    def self.on_focusin = "onfocusin"
    def self.on_focus_out = "onfocusout"
    def self.on_hash_change = "onhashchange"
    def self.on_input = "oninput"
    def self.on_invalid = "oninvalid"
    def self.on_key_down = "onkeydown"
    def self.on_key_press = "onkeypress"
    def self.on_key_up = "onkeyup"
    def self.on_load = "onload"
    def self.on_loaded_data = "onloadeddata"
    def self.on_loaded_meta_data = "onloadedmetadata"
    def self.on_load_start = "onloadstart"
    def self.on_mouse_down = "onmousedown"
    def self.on_mouse_enter = "onmouseenter"
    def self.on_mouse_leave = "onmouseleave"
    def self.on_mouse_move = "onmousemove"
    def self.on_mouse_over = "onmouseover"
    def self.on_mouse_out = "onmouseout"
    def self.on_mouse_up = "onmouseup"
    def self.on_offline = "onoffline"
    def self.on_online = "ononline"
    def self.on_page_hide = "onpagehide"
    def self.on_page_show = "onpageshow"
    def self.on_paste = "onpaste"
    def self.on_pause = "onpause"
    def self.on_play = "onplay"
    def self.on_playing = "onplaying"
    def self.on_progress = "onprogress"
    def self.on_rate_change = "onratechange"
    def self.on_resize = "onresize"
    def self.on_reset = "onreset"
    def self.on_scroll = "onscroll"
    def self.on_search = "onsearch"
    def self.on_seeked = "onseeked"
    def self.on_seeking = "onseeking"
    def self.on_select = "onselect"
    def self.on_stalled = "onstalled"
    def self.on_submit = "onsubmit"
    def self.on_suspend = "onsuspend"
    def self.on_time_update = "ontimeupdate"
    def self.on_toggle = "ontoggle"
    def self.on_touch_cancel = "ontouchcancel"
    def self.on_touchend = "ontouchend"
    def self.on_touch_move = "ontouchmove"
    def self.on_touch_start = "ontouchstart"
    def self.on_unload = "onunload"
    def self.on_volume_change = "onvolumechange"
    def self.on_waiting = "onwaiting"
    def self.on_wheel = "onwheel"
  end

  class HtmlEventListener
    def self.abort = "abort"
    def self.after_print = "afterprint"
    def self.before_print = "beforeprint"
    def self.before_unload = "beforeunload"
    def self.blur = "blur"
    def self.can_play = "canplay"
    def self.can_play_through = "canplaythrough"
    def self.change = "change"
    def self.click = "click"
    def self.copy = "copy"
    def self.cut = "cut"
    def self.double_click = "dblclick"
    def self.drag = "drag"
    def self.drag_end = "dragend"
    def self.drag_enter = "dragenter"
    def self.drag_leave = "dragleave"
    def self.drag_over = "dragover"
    def self.drag_start = "dragstart"
    def self.drop = "drop"
    def self.duration_change = "durationchange"
    def self.ended = "ended"
    def self.error = "error"
    def self.focus = "focus"
    def self.focusin = "focusin"
    def self.focus_out = "focusout"
    def self.hash_change = "hashchange"
    def self.input = "input"
    def self.invalid = "invalid"
    def self.key_down = "keydown"
    def self.key_press = "keypress"
    def self.key_up = "keyup"
    def self.load = "load"
    def self.loaded_data = "loadeddata"
    def self.loaded_meta_data = "loadedmetadata"
    def self.load_start = "loadstart"
    def self.mouse_down = "mousedown"
    def self.mouse_enter = "mouseenter"
    def self.mouse_leave = "mouseleave"
    def self.mouse_move = "mousemove"
    def self.mouse_over = "mouseover"
    def self.mouse_out = "mouseout"
    def self.mouse_up = "mouseup"
    def self.offline = "offline"
    def self.online = "online"
    def self.page_hide = "pagehide"
    def self.page_show = "pageshow"
    def self.paste = "paste"
    def self.pause = "pause"
    def self.play = "play"
    def self.playing = "playing"
    def self.progress = "progress"
    def self.rate_change = "ratechange"
    def self.resize = "resize"
    def self.reset = "reset"
    def self.scroll = "scroll"
    def self.search = "search"
    def self.seeked = "seeked"
    def self.seeking = "seeking"
    def self.select = "select"
    def self.stalled = "stalled"
    def self.submit = "submit"
    def self.suspend = "suspend"
    def self.time_update = "timeupdate"
    def self.toggle = "toggle"
    def self.touch_cancel = "touchcancel"
    def self.touchend = "touchend"
    def self.touch_move = "touchmove"
    def self.touch_start = "touchstart"
    def self.unload = "unload"
    def self.volume_change = "volumechange"
    def self.waiting = "waiting"
    def self.wheel = "wheel"

    def self.animation_end = "animationend"
    def self.animation_iteration = "animationiteration"
    def self.animation_start = "animationstart"
    def self.context_menu = "contextmenu"
    def self.full_screen_change = "fullscreenchange"
    def self.full_screen_error = "fullscreenerror"
    def self.pop_state = "popstate"
    def self.transition_end = "transitionend"
    def self.storage = "storage"

    # Custom
    def self.scroll_bottom = "scrollbottom" # Need Call EnableScrollBottomEvent Method Before
    def self.element_reached = "elementreached" # Need Call EnableReachedElementEvent Method Before
  end

  class String
    def child(value)
      if self.length < 1
        return value
      end

      self + "|" + value
    end

    def parent
      if self.length < 1
        return self
      end

      if self.end_with?("|/") || self.end_with?("//")
        return self + '/'
      end

      self + "|/"
    end

    def criteria(value)
      if self.length < 1
        return value
      end

      self + "?" + value.gsub("|", "$[vb];").gsub("?", "$[qu];").gsub("=", "$[eq];")
    end

    def append_fetch_replace(search_value, value)
      fs = 28.chr

      text = self[1..-1]
      "@;" + search_value + fs + value + fs + text
    end

    def line_break(encode_line = false)
      encode = encode_line ? "$[sln];" : ""
      self.gsub("\r\n", encode).gsub("\n", encode).gsub("\r", encode)
    end

    # Converts Numbers to Strings
    def to_js_string
      "\"" + self + "\""
    end

    # Get JS Object Momentary 
    def to_js_object
      "$" + self
    end

    # Get JS Object Returned Value Once
    def to_js_return_object
      "$@" + self
    end
  end
end

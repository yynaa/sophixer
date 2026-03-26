---@param position number
function schedule_sequence(position)
  renoise.song().transport:set_scheduled_sequence(position)
end

---@param s number
---@param e number
function set_loop(s, e)
  renoise.song().transport.loop_sequence_range = {s, e}
end

function mute_track(track, mute)
  if mute then
    renoise.song():track(track):mute()
  else
    renoise.song():track(track):unmute()
  end
end

function mute_track_sequence_slot(track, seq, muted)
  renoise.song().sequencer:set_track_sequence_slot_is_muted(track, seq, muted)
end

function bypass_effect(track, effect, bypass)
  renoise.song():track(track):device(effect).is_active = not bypass
end

function set_effect_param_value(track, effect, param, value)
  renoise.song():track(track):device(effect):parameter(param).value = value
end

-- DEBUG
if true then
  renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Debug:Schedule Sequence 0",
	invoke = function()
    schedule_sequence(1)
	end,
  })
  
  renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Debug:Set Loop from 1 to 3",
	invoke = function()
    set_loop(2, 4)
	end,
  })
  
  renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Debug:Mute Track 2 Sequence 1",
	invoke = function()
	  mute_track_sequence_slot(2, 2, true)
	end,
  })
  
  renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Debug:Bypass Effect 2 on Track 1",
	invoke = function()
    bypass_effect(1, 2, true)
	end,
  })
  
  renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Debug:Mute Track 1 through Track Pre-mixer",
	invoke = function()
    set_effect_param_value(1, 1, 2, 0.)
	end,
  })
end

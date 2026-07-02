local playback_control = {}

---@param position integer
function playback_control.schedule_sequence(position)
	renoise.song().transport:set_scheduled_sequence(position+1)
end

---@param position integer
function playback_control.trigger_sequence(position)
	renoise.song().transport:trigger_sequence(position+1)
end

--- @param s integer
--- @param e integer
function playback_control.set_loop(s, e)
	renoise.song().transport.loop_sequence_range = { s+1, e+1 }
end

--- @param track integer
--- @param mute boolean
function playback_control.mute_track(track, mute)
	if mute then
		renoise.song():track(track):mute()
	else
		renoise.song():track(track):unmute()
	end
end

--- @param track integer
--- @param seq integer
--- @param muted boolean
function playback_control.mute_track_sequence_slot(track, seq, muted)
	renoise.song().sequencer:set_track_sequence_slot_is_muted(track, seq + 1, muted)
end

--- @param track integer
--- @param effect integer
--- @param bypass boolean
function playback_control.bypass_effect(track, effect, bypass)
	renoise.song():track(track):device(effect).is_active = not bypass
end

--- @param track integer
---@param effect integer
---@param param integer
---@param value number
function playback_control.set_effect_param_value(track, effect, param, value)
	renoise.song():track(track):device(effect):parameter(param).value = value
end

function playback_control.stop_transport()
	renoise.song().transport:stop()
end

--- @param bpm number
function playback_control.set_bpm(bpm)
  renoise.song().transport.bpm = bpm
end

--- @param volume number
function playback_control.set_master_volume(volume)
  local master = renoise.song():track(renoise.song().sequencer_track_count + 1)
  master.postfx_volume.value = volume
end

-- DEBUG
if DEBUG then
	renoise.tool():add_menu_entry({
		name = "Main Menu:Tools:Calcium:Debug:Schedule Sequence 0",
		invoke = function()
			playback_control.schedule_sequence(1)
		end,
	})

	renoise.tool():add_menu_entry({
		name = "Main Menu:Tools:Calcium:Debug:Set Loop from 1 to 3",
		invoke = function()
			playback_control.set_loop(2, 4)
		end,
	})

	renoise.tool():add_menu_entry({
		name = "Main Menu:Tools:Calcium:Debug:Mute Track 2 Sequence 1",
		invoke = function()
			playback_control.mute_track_sequence_slot(2, 2, true)
		end,
	})

	renoise.tool():add_menu_entry({
		name = "Main Menu:Tools:Calcium:Debug:Bypass Effect 2 on Track 1",
		invoke = function()
			playback_control.bypass_effect(1, 2, true)
		end,
	})

	renoise.tool():add_menu_entry({
		name = "Main Menu:Tools:Calcium:Debug:Mute Track 1 through Track Pre-mixer",
		invoke = function()
			playback_control.set_effect_param_value(1, 1, 2, 0.)
		end,
	})
end

return playback_control

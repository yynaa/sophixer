local playback_control = require("playback_control")

local MessageFromRenoise = require("messages.message_from_renoise")
local MessageToRenoise = require("messages.message_to_renoise")

class "Client"
  function Client:__init()
  	self.socket = renoise.Socket.create_client("localhost", 3000, 2)
    self.connected = false
    self:send(MessageFromRenoise.Hello.new():_wrap())
    renoise.app():show_status("attempting to connect to tin... is tin running?")
  end

  function Client:destroy()
    self:send(MessageFromRenoise.Goodbye.new():_wrap())
  	self.socket = nil
    self.connected = false
  end

  --- @param msg MessageFromRenoise
  function Client:send(msg)
  	if self.socket then
      print("sending: " .. msg.command._id)
  		local success, error = self.socket:send(string.char(2) .. msg:_serialize())
  		if not success then
  			warn("couldn't send message to server: " .. error)
  		end
  	end
  end

  --- @param msg MessageToRenoise
  function Client:handle_message(msg)
    -- ID: 1
    if msg.command._id == MessageToRenoise.Welcome.id then
      renoise.app():show_status("connected to tin!")
    end
    
    -- ID: 3
    if msg.command._id == MessageToRenoise.PlaySection.id then
      if msg.command.force_play then
        playback_control.trigger_sequence(msg.command.section)
      else
        playback_control.schedule_sequence(msg.command.section)
      end
    end

    -- ID: 4
    if msg.command._id == MessageToRenoise.SetLoop.id then
      playback_control.set_loop(msg.command.loop_start, msg.command.loop_end)
    end

    -- ID: 5
    if msg.command._id == MessageToRenoise.StopTransport.id then
      playback_control.stop_transport()
    end

    -- ID: 6
    if msg.command._id == MessageToRenoise.MuteTrack.id then
      playback_control.mute_track(msg.command.track, msg.command.mute)
    end

    -- ID: 7
    if msg.command._id == MessageToRenoise.MuteTrackSequenceSlot.id then
      playback_control.mute_track_sequence_slot(msg.command.track, msg.command.sequence, msg.command.bypass)
    end

    -- ID: 8
    if msg.command._id == MessageToRenoise.BypassEffect.id then
      playback_control.bypass_effect(msg.command.track, msg.command.effect, msg.command.bypass)
    end

    -- ID: 9
    if msg.command._id == MessageToRenoise.SetParameterValue.id then
      playback_control.set_effect_param_value(
        msg.command.track,
        msg.command.effect,
        msg.command.parameter,
        msg.command.value
      )
    end

    -- ID: 10
    if msg.command._id == MessageToRenoise.SetBpm.id then
      playback_control.set_bpm(msg.command.bpm)
    end

    -- ID: 11
    if msg.command._id == MessageToRenoise.SetMasterVolume.id then
      playback_control.set_master_volume(msg.command.volume)
    end
  end
  
  function Client:callback()
    if self.socket then
      ---@type string|nil
      ---@diagnostic disable-next-line: assign-type-mismatch
      local s, _ = self.socket:receive("*all", 1)
      if s then
        while true do
          local msg, length = MessageToRenoise.deserialize(s)
          if msg then
            print("received: " .. msg.command._id)
            self:handle_message(msg)
            s = s:sub(length + 1)
            if #s == 0 then
              break
            end
          else
            print("unrecognized message at end of queue: " .. string.to_bytes(s))
            break
          end
        end
      end
    end
  end

  --[[
  local sub = string.split(msg, ",")
  if #sub == 1 then
    if sub[1] == "welcome" then
      renoise.app():show_status("connected to tin!")
    elseif sub[1] == "stopTransport" then
      renoise.song().transport:stop()
    end
  elseif #sub == 2 then
    if sub[1] == "setBPM" then
      local bpm = tonumber(sub[2])
      if bpm ~= nil then
        renoise.song().transport.bpm = bpm
      end
    elseif sub[1] == "setMasterVolume" then
      local vol = tonumber(sub[2])
      if vol ~= nil then
        local master = renoise.song():track(renoise.song().sequencer_track_count + 1)
        master.postfx_volume.value = vol
      end
    end
  elseif #sub == 3 then
    if sub[1] == "muteTrack" then
      local track = tonumber(sub[2])
      local mute = sub[3] == "1"
      if track ~= nil then
        playback_control.mute_track(track, mute)
      else
        warn("invalid track number (NaN)")
      end
    elseif sub[1] == "playSection" then
      local seq = tonumber(sub[2])
      local force = sub[3] == "1"
      if seq ~= nil then
        if force then
          playback_control.trigger_sequence(seq)
        else
          playback_control.schedule_sequence(seq)
        end
      else
        warn("invalid section numbers (NaN)")
      end
    elseif sub[1] == "setLoop" then
      local loop_start = tonumber(sub[2])
      local loop_end = tonumber(sub[3])
      if loop_start ~= nil and loop_end ~= nil then
        playback_control.set_loop(loop_start, loop_end)
      else
        warn("invalid track number (NaN)")
      end
    end
  elseif #sub == 4 then
    if sub[1] == "muteTrackSequenceSlot" then
      local track = tonumber(sub[2])
      local seq = tonumber(sub[3])
      local mute = sub[4] == "1"
      if seq ~= nil and track ~= nil then
        playback_control.mute_track_sequence_slot(track, seq, mute)
      else
        warn("invalid track and seq numbers (NaN)")
      end
    elseif sub[1] == "bypassEffect" then
      local track = tonumber(sub[2])
      local effect = tonumber(sub[3])
      local bypass = sub[4] == "1"
      if effect ~= nil and track ~= nil then
        playback_control.bypass_effect(track, effect, bypass)
      else
        warn("invalid track and seq numbers (NaN)")
      end
    end
  elseif #sub == 5 then
    if sub[1] == "setParameterValue" then
      local track = tonumber(sub[2])
      local effect = tonumber(sub[3])
      local param = tonumber(sub[4])
      local value = tonumber(sub[5])
      if effect ~= nil and track ~= nil and param ~= nil and value ~= nil then
        playback_control.set_effect_param_value(track, effect, param, value)
      else
        warn("invalid track and seq numbers (NaN)")
      end
    end
  end
  ]]

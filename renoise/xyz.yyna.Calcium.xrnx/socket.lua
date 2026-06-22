class "Client"
  function Client:__init()
  	self.socket = renoise.Socket.create_client("localhost", 3000, 2)
    self.connected = false
    self:send("hello")
    renoise.app():show_status("attempting to connect to tin... is tin running?")
  end

  function Client:destroy()
    self:send("goodbye")
  	self.socket = nil
    self.connected = false
  end

  function Client:send(msg)
  	if self.socket then
      print("sending: " .. msg)
  		local success, error = self.socket:send("calcium:" .. msg .. ";")
  		if not success then
  			warn("couldn't send message to server: " .. error)
  		end
  	end
  end
  
  function Client:handle_message(msg)
    local sub = string_split(msg, ",")
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
          mute_track(track, mute)
        else
          warn("invalid track number (NaN)")
        end
      elseif sub[1] == "playSection" then
        local seq = tonumber(sub[2])
        local force = sub[3] == "1"
        if seq ~= nil then
          if force then
            trigger_sequence(seq)
          else
            schedule_sequence(seq)
          end
        else
          warn("invalid section numbers (NaN)")
        end
      elseif sub[1] == "setLoop" then
        local loop_start = tonumber(sub[2])
        local loop_end = tonumber(sub[3])
        if loop_start ~= nil and loop_end ~= nil then
          set_loop(loop_start, loop_end)
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
          mute_track_sequence_slot(track, seq, mute)
        else
          warn("invalid track and seq numbers (NaN)")
        end
      elseif sub[1] == "bypassEffect" then
        local track = tonumber(sub[2])
        local effect = tonumber(sub[3])
        local bypass = sub[4] == "1"
        if effect ~= nil and track ~= nil then
          bypass_effect(track, effect, bypass)
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
          set_effect_param_value(track, effect, param, value)
        else
          warn("invalid track and seq numbers (NaN)")
        end
      end
    end
  end
  
  function Client:callback()
    if self.socket then
      ---@type string|nil
      ---@diagnostic disable-next-line: assign-type-mismatch
      local s, _ = self.socket:receive("*all", 1)
      if (s) then
        local messages = string_split(s, ";")
        for _, msg in ipairs(messages) do
          if #msg > 0 then
            print("received: " .. msg)
            self:handle_message(msg)
          end
        end
      end
    end
  end

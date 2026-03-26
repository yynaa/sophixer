class "Client"
  function Client:__init()
  	self.socket = renoise.Socket.create_client("localhost", 3000, 2)
    self.connected = false
    self:send("hello")
    self.queue = {}
  end

  function Client:destroy()
    self:send("goodbye")
  	self.socket = nil
    self.connected = false
  end

  function Client:send(msg)
  	if self.socket then
      print("sending " .. msg)
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
        renoise.app():show_prompt("Calcium connected", "Calcium connected", { "OK" })
      elseif sub[1] == "stopTransport" then
        renoise.song().transport:stop()
      end
    elseif #sub == 2 then
      if sub[1] == "loadSong" then
        print("load song: " .. sub[2])
        renoise.app():load_song(sub[2])
        loading_song = true
      end
    elseif #sub == 3 then
      if sub[1] == "playSection" then
        local seq = tonumber(sub[2])
        local length = tonumber(sub[3])
        if seq ~= nil and length ~= nil then
          schedule_sequence(seq)
          set_loop(seq, seq + length - 1)
        else
          warn("invalid section numbers (NaN)")
        end
      elseif sub[1] == "muteTrack" then
        local track = tonumber(sub[2])
        local mute = sub[3] == "1"
        if track ~= nil then
          mute_track(track, mute)
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
    if self.socket and not loading_song then
      if #self.queue > 0 then
        print("queue")
        for _, msg in ipairs(self.queue) do
          self:handle_message(msg)
        end
        self.queue = {}
      end
      
      ---@type string|nil
      ---@diagnostic disable-next-line: assign-type-mismatch
      local s, _ = self.socket:receive("*all", 1)
      if (s) then
        local messages = string_split(s, ";")
        for _, msg in ipairs(messages) do
          if #msg > 0 then
            if loading_song then
              table.insert(self.queue, msg)
            else
              self:handle_message(msg)
            end
          end
        end
      end
    end
  end

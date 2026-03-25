class "Client"
  function Client:__init()
  	self.socket = renoise.Socket.create_client("localhost", 3000, 2)
    self.connected = false
    self:send("hello")
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
  
  function Client:callback()
    if self.socket then
      ---@type string|nil
      ---@diagnostic disable-next-line: assign-type-mismatch
      local s, _ = self.socket:receive("*all", 1)
      if (s) then
        local messages = string_split(s, ";")
        for _, msg in ipairs(messages) do
          if #msg > 0 then
            local sub = string_split(msg, ",")
            if #sub == 1 then
              if sub[1] == "welcome" then
                renoise.app():show_prompt("Calcium connected", "Calcium connected", { "OK" })
              end
            end
          end
        end
      end
    end
  end

--- @param str string
--- @param delimiter string
--- @return string[]
function string.split(str, delimiter)
  local result = { }
  local from  = 1
  local delim_from, delim_to = string.find( str, delimiter, from  )
  while delim_from do
    table.insert( result, string.sub( str, from , delim_from-1 ) )
    from  = delim_to + 1
    delim_from, delim_to = string.find( str, delimiter, from  )
  end
  table.insert( result, string.sub( str, from  ) )
  return result
end

--- @param s string
--- @return string
function string.to_bytes(s)
  local parts = {}
  for i = 1, #s do
    parts[#parts + 1] = string.format("%02x", s:byte(i))
  end
  return table.concat(parts, " ")
end

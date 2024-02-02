local f = function(line)
  if string.find(line, 'hello') then
    return 'Hello!'
  elseif string.find(line, 'bye') then
    return { 'So long!', 'Great game!' }
  end
end

return { f = f }

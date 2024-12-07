-- Lua script to launch Spotify
print("--> Attempting to launch Spotify <--")

local result = os.execute("spotify")

if result then
    print("--> Spotify launched successfully <--")
else
    print("--> Failed to launch Spotify. Check your PATH or installation. <--")
end

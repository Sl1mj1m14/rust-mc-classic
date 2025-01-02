# rust-mc-classic
  Contains functionality for reading and ~~writing~~ Minecraft Classic & Pre-classic worlds. 

## How Does Minecraft Classic Store Files?
  Minecraft classic technically stores files in 2 main formats, these being `.dat` and `.mine`. I say technically here because in actuality these are identical file formats with just different extensions. Going further in depth, the actual file format for both is a gzip stream of bytes, and depending on the version the world is saved in, there are 3 major formats.

### Format #1 ###
  From Pre-Classic 132011 Through Classic 0.0.12a_03 classic files only store the world blocks, in the format of XZY

### Format #2 ###
  From Classic 0.13a-dev Through Classic 0.13a_03 classic files follow the following format:
  


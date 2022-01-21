from dslake.img import NCLR
import json

backbeat = NCLR(open("test_files/backbeat.NCLR", "rb").read())
backbeat.palettes[0] = backbeat.palettes[0].__dict__
for i in range(len(backbeat.palettes[0]["colors"])):
    backbeat.palettes[0]["colors"][i] = backbeat.palettes[0]["colors"][i].__dict__
out = open("test_files/backbeat.NCLR.json", "w")
out.write(json.dumps(backbeat.__dict__, indent=4))
out.close()
# Odin project

**topic format**
/_sender_id_/_type_/_receiver_id_/

# API
**SWITCH**<br/>
/_sender_id_/**switch**/_receiver_id_/
payload |1 byte| u8<br/>
0x01 - on action<br/>
0x02 - off action<br/>
0x03 - toggle action<br/>

**Spot**<br/>

#Spot
/_sender_id_/**spot**/_spot_id_/
payload |1 byte| u8<br/>
|1 bit (0x01 - on;0x01 - off)|7 bit (0-100 brightnesses)|<br/>

 


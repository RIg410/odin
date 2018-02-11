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

**SPOT**<br/>
/_sender_id_/**spot**/_spot_id_/
payload |1 byte| u8<br/>
|1 bit (0x01 - on;0x01 - off)|7 bit (0-100 brightnesses)|<br/>


**TAP**<br/>
/sender_id/**tap**/tap_id/ payload |1 byte| u8<br/>
0x00 - water off<br/>
0x01 - water on<br/>

**LEAKS SENSOR**<br/>
/sender_id/**leak_sensor**/leak_sensor_id/ payload |1 byte| u8<br/>
0x00 - no leaks found<br/>
0x01 - found the leak<br/>
 


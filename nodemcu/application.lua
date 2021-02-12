uart.setup(0, 115200, 8, uart.PARITY_NONE, uart.STOPBITS_1, 0)

m = mqtt.Client("alarm", 120)

function handle_mqtt_error(client, reason)
    tmr.create():alarm(1000, tmr.ALARM_SINGLE, mqtt_connect)
end

function mqtt_connect()
    m:connect("mosquitto", 1883, false, function(client) print("connected to MQTT") end, handle_mqtt_error)
end
mqtt_connect()

uart.on("data", ";", function(data)
    start = string.sub(data, 0, 1)
    if start == 'P' then
        pin, status = data:match("([^:]+):([^:]+);")
        m:publish("alarm/sensor/" .. pin, status, 0, 0, function(client) print("sent: " .. pin) end)
    end
end, 0)

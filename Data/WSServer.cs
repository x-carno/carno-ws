using System;
using AltWebSocketSharp;
using AltWebSocketSharp.Server;


namespace carno_ws.Data
{
    public class WSServer
    {
        public WebSocketServer wssv = new WebSocketServer(5001);

        public WSServer()
        {
            Console.WriteLine("start websocket server...");
            wssv.AddWebSocketService<WSClientService>("/ws");
            wssv.Start();
        }
    }
}


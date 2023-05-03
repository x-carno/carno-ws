namespace carno_ws.Data;

using System.Collections;
using System.Collections.Generic;
using System.Linq;
using AltWebSocketSharp;
using AltWebSocketSharp.Server;
using carno_ws.Shared;

//     @* @foreach (var c in WSClientService.clientList)
// {
// <MudCheckBox @bind-Checked="@c.Checked" Color="Color.Primary" UnCheckedColor="Color.Default">
// @c.Addr
// </MudCheckBox>
// } *@

public class WSClient
{
    public string? sessionId { get; set; }
    public string? Addr { get; set; }
    public bool Checked { get; set; }
    public string? SecWebSocketKey { get; set; }
}

public delegate void WSClientChangeEventDelegate(object sender);

public class WSClientChangeEventArgs : EventArgs
{

}

public class WSClientService : WebSocketBehavior
{
    private static WSClientService? _instance;
    private static readonly object _lock = new object();
    public static WSClientService GetInstance()
    {
        if (_instance == null)
        {
            lock (_lock)
            {
                if (_instance == null)
                {
                    _instance = new WSClientService();
                }
            }
        }
        return _instance;
    }

    public event WSClientChangeEventDelegate? OnWSClientChange;

    public static List<WSClient> clientList = new List<WSClient>();
    public static List<WSMessage> communicationMessages = new List<WSMessage>();

    public int ClientsCount { get; set; }
    private static WebSocketSessionManager? mySession { get; set; }

    public WSClientService()
    {
        var now = System.DateTime.Now.ToLongTimeString();
        Console.WriteLine($"init websocket client service instance at {now}");
    }

    public Task<List<WSClient>> GetWSClientsAsync()
    {
        // Func<int, WSClient> selector = index =>
        // {
        //     return new WSClient
        //     {
        //         Id = index,
        //         Addr = "local",
        //         Checked = false,
        //     };
        // };

        // this.clientList = Enumerable.Range(1, 5).Select(selector).ToList();

        return Task.FromResult(clientList);
    }

    public void SendMessage(String message)
    {
        foreach (var c in clientList)
        {
            if (c.Checked)
            {
                mySession!.SendTo(message, c.sessionId);
                // base.Sessions.SendTo(message, c.sessionId);
            }
        }
    }

    protected override void OnOpen()
    {
        var newId = base.ID;
        Console.WriteLine($"connect id is {newId}");
        var headers = base.Headers;
        var allKeys = headers.AllKeys;
        foreach (var k in allKeys)
        {
            Console.WriteLine($"key: {k} --- val: {headers.Get(k)}");
        }
        var newClient = new WSClient
        {
            sessionId = base.ID,
            Checked = false,
            Addr = headers.Get("Host"),
        };
        clientList.Add(newClient);
        Console.WriteLine($"add client {newId}");
        Console.WriteLine($"current client count {clientList.Count()}");

        GetInstance().OnWSClientChange!(this);
        mySession = base.Sessions;
    }

    protected override void OnMessage(MessageEventArgs e)
    {
        base.OnMessage(e);

        var msg = new WSMessage();
        msg.IsSendOrReceive = false;
        msg.message = e.Data;
        communicationMessages.Add(msg);
        GetInstance().OnWSClientChange!(this);

        // Console.WriteLine(e.Data.ToString());
        // foreach (var c in clientList)
        // {
        //     // Sessions.SendTo(e.Data, c.sessionId);
        //     if (c.Checked)
        //     {
        //         // Sessions.SendTo(e.Data, c.sessionId);
        //         mySession!.SendTo($"echo from server at {System.DateTime.Now.ToLongTimeString()}", c.sessionId);
        //     }
        // }
    }

    protected override void OnError(ErrorEventArgs e)
    {
        base.OnError(e);

        Console.WriteLine($"error: {e.Message}");
    }

    protected override void OnClose(CloseEventArgs e)
    {

        int rm = -1;
        for (int i = 0; i < clientList.Count; i++)
        {
            if (clientList[i].sessionId == base.ID)
            {
                rm = i;
            }
        }

        if (rm != -1)
        {
            clientList.RemoveAt(rm);
            GetInstance().OnWSClientChange!(this);
        }
    }
}
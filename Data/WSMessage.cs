using System;
namespace carno_ws.Data
{

    // @foreach (var m in WSClientService.communicationMessages)
    //                 {
    //                     <MudDivider />
    //                     @if (m.IsSendOrReceive)
    //                     {
    //                         <MudListItem Text="@m.message" Icon="@Icons.Material.Filled.CallMade" />
    //                     }
    //                     else
    //                     {
    //                         <MudListItem Text="@m.message" Icon="@Icons.Material.Filled.CallReceived" />
    //                     }
    //                 }

    public class WSMessage
    {
        // true is send, false is receive
        public bool IsSendOrReceive { get; set; }
        public string? message { get; set; }

        public WSMessage()
        {
        }
    }
}


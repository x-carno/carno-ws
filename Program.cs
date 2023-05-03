using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using carno_ws.Data;
using MudBlazor.Services;

namespace carno_ws;

public class Program
{
    public static void Main(string[] args)
    {
        WSServer wss = new WSServer();

        var builder = WebApplication.CreateBuilder(args);

        // Add services to the container.
        builder.Services.AddRazorPages();
        builder.Services.AddServerSideBlazor();
        builder.Services.AddSingleton<WeatherForecastService>();
        builder.Services.AddSingleton<WSClientService>();
        builder.Services.AddMudServices();

        var app = builder.Build();

        // Configure the HTTP request pipeline.
        if (!app.Environment.IsDevelopment())
        {
            app.UseExceptionHandler("/Error");
        }

        app.UseWebSockets(new WebSocketOptions
        {
            KeepAliveInterval = TimeSpan.FromMinutes(2),
        });

        app.UseStaticFiles();

        app.UseRouting();

        app.MapBlazorHub();
        app.MapFallbackToPage("/_Host");

        app.Run();
    }
}


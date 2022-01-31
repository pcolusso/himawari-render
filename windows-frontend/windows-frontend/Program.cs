using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.IO;
using System.Windows.Forms;
using System.Runtime.InteropServices;
using Microsoft.Win32;
using Microsoft.DirextX.Direct3D;

namespace windows_frontend
{
    static class Program
    {
        
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);

            Manager manager = new Manager();

            Application.Run(new CustomApplicationContext(manager));

            //
            //Console.WriteLine(result);
        }
    }

    public class CustomApplicationContext : ApplicationContext
    {
        private NotifyIcon icon;
        private Manager manager;
        private Timer timer;

        public CustomApplicationContext(Manager manager)
        {
            icon = new NotifyIcon()
            {
                Icon = Resources.AppIcon,
                ContextMenu = new ContextMenu(new MenuItem[]
                {
                    new MenuItem("Exit", Exit),
                    new MenuItem("Refresh Now", RefreshNow)
                }),
                Visible = true
            };

            this.manager = manager;
            timer = new Timer();
            timer.Interval = 1000 * 60 * 10;
            timer.Tick += new EventHandler(RefreshNow);
            timer.Start();
        }

        void Exit(object sender, EventArgs e)
        {
            icon.Visible = false;
            Application.Exit();
        }

        void RefreshNow(object sender, EventArgs e)
        {
            manager.CreateWallpaper();
            manager.SetWallpaper();
        }
    }

    public class Manager
    {
        [DllImport("himawari_render.dll")]
        private static extern String single_wallpaper_file(UInt32 width, UInt32 height, UInt32 quality, String path);

        [DllImport("user32.dll", CharSet = CharSet.Auto)]
        private static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);

        private UInt32 width, height, quality;
        private string path;
        private bool isGenerated;

        public Manager()
        {
            width = 3840;
            height = 2160;
            quality = 2;
            isGenerated = false;
            path = Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
                "himawari_wallpaper.jpg"
            );
        }

        public String CreateWallpaper()
        {
            var result = single_wallpaper_file(width, height, quality, path);
            if (result == "Success")
            {
                isGenerated = true;
            }
            return result;
            
        }

        const int SPI_SETDESKWALLPAPER = 20;
        const int SPIF_UPDATEINIFILE = 0x01;
        const int SPIF_SENDWININICHANGE = 0x02;

        public void SetWallpaper()
        {
            if (!isGenerated) {
                return;
            }

            RegistryKey key = Registry.CurrentUser.OpenSubKey(@"Control Panel\Desktop", true);
            key.SetValue(@"WallpaperStyle", 1.ToString());
            key.SetValue(@"TileWallpaper", 0.ToString());

            SystemParametersInfo(SPI_SETDESKWALLPAPER, 0, path, SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE);
        }
    }
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Runtime.InteropServices;

namespace windows_frontend
{
  static class Program
  {
    [DllImport("himawari_render.dll")]
    private static extern String save_planet(String path, Int32 level);
    /// <summary>
    /// The main entry point for the application.
    /// </summary>
    [STAThread]
    static void Main()
    {
      Application.EnableVisualStyles();
      Application.SetCompatibleTextRenderingDefault(false);

      save_planet("out.png", 4);
    }
  }
}

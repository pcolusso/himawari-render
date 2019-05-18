using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace windows_frontend
{
  public partial class TrayView : Form
  {
    public TrayView()
    {
      InitializeComponent();
    }

    private void TrayView_Load(object sender, EventArgs e)
    {

    }

    private void Icon_MouseDoubleClick(object sender, EventArgs e)
    {
      MessageBox.Show("Doing something important on double-click...");
    }
  }
}

using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace TexturePackager
{
    public partial class MainForm : Form
    {
        public TextureFactory textureFactory;
        public int width = 3000;
        public int height = 1969;
        public MainForm()
        {
            InitializeComponent();
        }

        private void MainForm_Load(object sender, EventArgs e)
        {
            textureFactory = new TextureFactory(width, height);
        }

        private void updateListBox()
        {
            textureListBox.Items.Clear();
            foreach (Texture texture in textureFactory.textures.Values)
            {
                textureListBox.Items.Add(texture.key);
            }
            textureListBox.Refresh();
        }

        public void updateImageBox()
        {
            imagePreviewBox.Image = textureFactory.image;
            imagePreviewBox.Refresh();
        }

        private void addEditButton_Click(object sender, EventArgs e)
        {
            string key = keyText.Text;
            string path = pathText.Text;
            textureFactory.newTexture(path, key);
            pathText.Clear();
            keyText.Clear();
            updateImageBox();
            updateListBox();
        }

        private void browseButton_Click(object sender, EventArgs e)
        {
            if(openImageDialog.ShowDialog() == DialogResult.OK)
            {
                pathText.Text = openImageDialog.FileName;
            }

        }

        private void exportButton_Click(object sender, EventArgs e)
        {

        }

        private void imagePreviewBox_Click(object sender, EventArgs e)
        {
            
        }

        private void listBox1_SelectedIndexChanged(object sender, EventArgs e)
        {
            keyText.Text = (string)this.textureListBox.SelectedItem;
        }

        private void removeImage_Click(object sender, EventArgs e)
        {
            string key = (string)this.textureListBox.SelectedItem;
            this.textureFactory.removeTexture(key);
            updateImageBox();
            updateListBox();
        }

        private void saveButton_Click(object sender, EventArgs e)
        {
            if(saveImgPackDlg.ShowDialog() == DialogResult.OK)
            {
                PackageFile packageFile = new PackageFile(saveImgPackDlg.FileName);
                packageFile.SaveFile(saveImgPackDlg.FileName, textureFactory);
            }
        }

        private void openButton_Click(object sender, EventArgs e)
        {
            if(openImgPackDlg.ShowDialog() == DialogResult.OK)
            {
                PackageFile packageFile = new PackageFile(saveImgPackDlg.FileName);
                textureFactory = packageFile.OpenFile(openImgPackDlg.FileName);
            }
            updateListBox();
            updateImageBox();
        }
    }
}

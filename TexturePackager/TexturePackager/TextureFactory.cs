using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace TexturePackager
{
    public class TextureFactory
    {
        public Dictionary<String, Texture> textures;
        public Bitmap image;
        public int width;
        public int height;

        public TextureFactory(int width, int height)
        {
            textures = new Dictionary<string, Texture>();
            image = new Bitmap(width, height);
            this.width = width;
            this.height = height;
        }

        public void newTexture(string fname, string key)
        {
            Bitmap newImage = new Bitmap(fname);
            if (newImage.Width > this.image.Width && newImage.Height > this.image.Height)
                throw new Exception("Width and height too high");
            if (textures.ContainsKey(key))
                textures[key] = new Texture(newImage, key);
            else
                textures.Add(key, new Texture(newImage, key));
            packImages();
            recreateImage();
        }

        public void textureWithPos(string key, int x, int y, int w, int h)
        {
            Bitmap texture = image.Clone(new Rectangle(x, y, w, h), System.Drawing.Imaging.PixelFormat.Format32bppPArgb);
            Texture curTexture = new Texture(key, texture, x, y);
            textures.Add(key, curTexture);
        }

        public void removeTexture(string key)
        {
            if (textures.ContainsKey(key))
            {
                textures.Remove(key);
                packImages();
                recreateImage();
            }
        }

        public void recreateImage()
        {
            this.image = new Bitmap(width, height);
            using (Graphics g = Graphics.FromImage(image))
            {
                foreach (Texture texture in textures.Values)
                {
                    g.DrawImage(texture.texture, new Point(texture.x, texture.y));
                }
            }
        }

        public void packImages()
        {
            List<Texture> textureList = textures.Values.ToList();
            List<Texture> textureResults = new List<Texture>();
            int height = 0;
            while (textureList.Count > 0)
            {
                int width = 0;
                int maxHeight = 0;
                textureList = (from curTexture in textureList orderby curTexture.texture.Width descending select curTexture).ToList();
                foreach(Texture curTexture in textureList.ToList())
                {
                    curTexture.x = width;
                    curTexture.y = height;
                    width += curTexture.texture.Width;
                    if(width > this.image.Width)
                    {
                        height += maxHeight;
                        break;
                    }
                    else
                    {
                        if (curTexture.texture.Height > maxHeight)
                            maxHeight = curTexture.texture.Height;
                        textureResults.Add(new Texture(curTexture));
                        textureList.Remove(curTexture);
                    }
                }
            }
            this.textures = new Dictionary<string, Texture>();
            foreach(Texture curTexture in textureResults)
            {
                textures.Add(curTexture.key, curTexture);
            }
            
        }


    }

    public class Texture
    {
        public Bitmap texture;
        public string key;
        public int x;
        public int y;

        public Texture(string fname, string key)
        {
            this.texture = new Bitmap(fname);
            this.key = key;
        }

        public Texture(Texture texture)
        {
            this.texture = new Bitmap(texture.texture);
            this.x = texture.x;
            this.y = texture.y;
            this.key = texture.key;
        }

        public Texture(string key, Bitmap texture, int x, int y)
        {
            this.key = key;
            this.texture = texture;
            this.x = x;
            this.y = y;
        }

        public Texture(Bitmap bitmap, string key)
        {
            this.texture = bitmap;
            this.key = key;
        }
    }
}

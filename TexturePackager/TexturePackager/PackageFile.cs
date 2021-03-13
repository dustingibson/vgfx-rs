using System;
using System.IO;
using System.ComponentModel;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;
using System.Linq;

namespace TexturePackager
{
    class PackageFile
    {
        public string fname;
        public PackageFile(string fname)
        {
            this.fname = fname;
        }

        public void SaveFile(string fname, TextureFactory textureFactory)
        {
            // Number of items
            // Width
            // Height
            // Image size
            // Image
            // ---- for each item ----
            // Key Size 
            // Key (string)
            // x
            // y
            // w
            // h

            List<byte> byteList = new List<byte>();
            byteList.AddRange( getIntBytes(textureFactory.textures.Count).ToList() );
            byteList.AddRange( getIntBytes(textureFactory.image.Width).ToList() );
            byteList.AddRange( getIntBytes(textureFactory.image.Height).ToList() );
            byte[] imageBytes = getImageBytes(textureFactory.image);
            byteList.AddRange(getIntBytes(imageBytes.Length));
            byteList.AddRange(imageBytes);
            
            foreach (Texture texture in textureFactory.textures.Values.ToList())
            {
                byteList.AddRange(getIntBytes(texture.key.Length));
                byteList.AddRange(getStringBytes(texture.key));
                byteList.AddRange(getIntBytes(texture.x));
                byteList.AddRange(getIntBytes(texture.y));
                byteList.AddRange(getIntBytes(texture.texture.Width));
                byteList.AddRange(getIntBytes(texture.texture.Height));
            }
            File.WriteAllBytes(fname, byteList.ToArray());
        }

        public TextureFactory OpenFile(string fname)
        {
            int cnt = 0;
            byte[] bytes = File.ReadAllBytes(fname);
            int n = getInt(bytes.Skip(cnt).Take(4).ToArray());
            cnt += 4;
            int width = getInt(bytes.Skip(cnt).Take(4).ToArray());
            cnt += 4;
            int height = getInt(bytes.Skip(cnt).Take(4).ToArray());
            cnt += 4;

            TextureFactory textureFactory = new TextureFactory(width, height);

            int imageSize = getInt(bytes.Skip(cnt).Take(4).ToArray());
            cnt += 4;
            textureFactory.image = getBitmap(bytes.Skip(cnt).Take(imageSize).ToArray());
            cnt += imageSize;

            for(int i = 0; i < n; i++)
            {
                int keySize = getInt(bytes.Skip(cnt).Take(4).ToArray());
                cnt += 4;
                string key = getString(bytes.Skip(cnt).Take(keySize).ToArray());
                cnt += keySize;
                int x = getInt(bytes.Skip(cnt).Take(4).ToArray());
                cnt += 4;
                int y = getInt(bytes.Skip(cnt).Take(4).ToArray());
                cnt += 4;
                int w = getInt(bytes.Skip(cnt).Take(4).ToArray());
                cnt += 4;
                int h = getInt(bytes.Skip(cnt).Take(4).ToArray());
                cnt += 4;

                textureFactory.textureWithPos(key, x, y, w, h);
            }
            return textureFactory;
        }

        public Bitmap getBitmap(byte[] value)
        {
            TypeConverter tc = TypeDescriptor.GetConverter(typeof(Bitmap));
            Bitmap finalBitmap = (Bitmap)tc.ConvertFrom(value);
            return finalBitmap;
        }

        public string getString(byte[] value)
        {
            return System.Text.Encoding.Default.GetString(value);
        }


        public byte[] getIntBytes(int newValue)
        {
            return BitConverter.GetBytes(newValue);
        }

        public byte[] getImageBytes(Bitmap newValue)
        {
            ImageConverter converter = new ImageConverter();
            return (byte[])converter.ConvertTo(newValue, typeof(byte[]));
        }

        public byte[] getStringBytes(string value)
        {
            return Encoding.ASCII.GetBytes(value);
        }

        public int getInt(byte[] value)
        {
            return BitConverter.ToInt32(value, 0);
        }
    }
}

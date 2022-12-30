import random
from PIL import Image

class Room:
    #Top, Right, Bottom, Right

    def __init__(self, id, size):
        self.id = id
        self.size = size
        if size == 0:
            self.width = 50
            self.height = 50
        elif size == 1:
            self.width = 100
            self.height = 100
        elif size == 2:
            self.width = 200
            self.height = 200
        self.adj_rooms = [-1, -1, -1, -1]
        self.draw_flag = False
        self.start_x = -1
        self.start_y = -1


class Floor:

    def __init__(self):
        self.rooms = []
        self.block_pixel = 25
        self.max_blocks = 100
        self.color_dict = {}
        self.focus_queue = []

        self.blocks = []
        for i in range(0, self.max_blocks):
            temp_blocks = []
            for j in range(0, self.max_blocks):
                temp_blocks.append(-1)
            self.blocks.append(temp_blocks)


    def create_room(self, id = None):
        if id == None:
            return Room(random.randint(0, 5000), random.randint(0, 2))
        else:
            return Room(id, random.randint(0,2))

    def fifty_fifty(self):
        return True
        # randInt = random.randint(0,1)
        # return randInt == 1

    def get_adj_side(self, side):
        # Top - 0
        # Right - 1
        # Bottom - 2
        # Left - 3
        if side == 0:
            return 2
        if side == 1:
            return 3
        if side == 2:
            return 0
        if side == 3:
            return 1

    def set_blocks(self, start_x, start_y, room):
        room.start_x = start_x
        room.start_y = start_y
        if str(room.id) not in self.color_dict:
            self.color_dict[str(room.id)] = (random.randint(0, 255), random.randint(0, 255), random.randint(0,255))
        for i in range(0, room.size + 1):
            for j in range(0, room.size + 1):
                if self.blocks[start_x + i][start_y + j] == -1:
                    self.blocks[start_x + i][start_y + j] = room.id
                else:
                    return False
        return True

    def get_random_focus_queue(self):
        if len(self.focus_queue) <= 0:
            return None
        idx = random.randint(0, len(self.focus_queue)-1)
        id = self.focus_queue[idx]
        self.focus_queue.remove(id)
        return self.find_room(id)

    def create_rooms(self, room_limit):
        #Create Initial Room
        cur_room = self.create_room(1)
        self.rooms.append(cur_room)
        rooms_created = 1
        start_x = int(self.max_blocks / 2)
        start_y = int(self.max_blocks / 2)
        self.set_blocks(start_x, start_y, cur_room)
        while rooms_created < room_limit:
            #Create Room
            for i in range(0, 4):
                if rooms_created >= room_limit:
                    return
                if self.fifty_fifty() and cur_room.adj_rooms[i] == -1:
                    id = rooms_created + 1
                    new_room = self.create_room(id)
                    new_room.adj_rooms[self.get_adj_side(i)] = cur_room.id
                    cur_room_size = cur_room.size + 1
                    new_room_size = new_room.size + 1
                    valid = False
                    # Top
                    if i == 0:
                        valid = self.set_blocks(start_x, start_y - new_room_size, new_room)
                    # Right
                    if i == 1:
                        valid = self.set_blocks(start_x + cur_room_size, start_y, new_room)
                    # Bottom
                    if i == 2:
                       valid = self.set_blocks(start_x + cur_room_size - new_room_size, start_y + cur_room_size, new_room)
                    # Left
                    elif i == 3:
                        valid = self.set_blocks(start_x - new_room_size, start_y + cur_room_size  - new_room_size, new_room)
                    if valid:
                        self.rooms.append(new_room)
                        self.focus_queue.append(new_room.id)
                        cur_room.adj_rooms[i] = new_room.id*1
                        rooms_created += 1
            cur_room = self.get_random_focus_queue()
            if cur_room == None:
                return
            start_x = cur_room.start_x
            start_y = cur_room.start_y
            

    def print_rooms(self):
        for room in self.rooms:
            print(f'Room ID {room.id}. Adj Rooms {room.adj_rooms}. Size {room.height}, {room.width}')
    
    def find_room(self, id):
        for room in self.rooms:
            if room.id == id:
                return room
        return room

    def plot_room(self, img, room, start_pnt, pos):
        for i in range(0, self.max_blocks):
            for j in range(0, self.max_blocks):
                if self.blocks[i][j] != -1:
                    for m in range(0, 25):
                        for n in range(0, 25):
                            img.putpixel((i*self.block_pixel + m, j*self.block_pixel - n), self.color_dict[str(self.blocks[i][j])])

    def plot_image(self):
        img = Image.new('RGB', (self.max_blocks*self.block_pixel, self.max_blocks*self.block_pixel))
        start_pnt = (self.max_blocks / 2, self.max_blocks / 2)
        self.plot_room(img, self.rooms[0], start_pnt, -1)
        img.save('output.png')


def create_blueprint():
    floor = Floor()
    floor.create_rooms(15)
    floor.print_rooms()
    floor.plot_image()

create_blueprint()
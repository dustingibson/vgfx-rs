import random
from PIL import Image

class RoomType:

    def __init__(self, id, name, size):
        self.id = id
        self.name = name
        self.size = size

class Room:
    #Top, Right, Bottom, Left

    def __init__(self, id, size, room_type):
        self.id = id
        self.size = room_type.size
        self.adj_rooms = [0, 0, 0, 0]
        self.draw_flag = False
        self.start_x = -1
        self.start_y = -1
        self.room_type = room_type


class Floor:

    def __init__(self):
        self.rooms = []
        self.block_pixel = 25
        self.max_blocks = 20
        self.color_dict = {}
        self.focus_queue = []

        self.blocks = []
        for i in range(0, self.max_blocks):
            temp_blocks = []
            for j in range(0, self.max_blocks):
                temp_blocks.append(0)
            self.blocks.append(temp_blocks)


    def create_room(self, id, room_type):
        if id == None:
            return Room(random.randint(0, 5000), random.randint(0, 2), room_type)
        else:
            return Room(id, random.randint(0,2), room_type)

    def random_rooms(self):
        out = []
        rand_room_id = random.randint(1, 15)        
        out.extend("{0:b}".format(rand_room_id).zfill(4))
        return out

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
                if self.blocks[start_x + i][start_y + j] != 0:
                    return False
        # I know I am repeating. Don't want to write to bitmap if invalid
        for i in range(0, room.size + 1):
            for j in range(0, room.size + 1):
                if self.blocks[start_x + i][start_y + j] == 0:
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

    def create_rooms(self, room_limit, room_types):
        #Create Initial Room
        cur_room = self.create_room(1, room_types[0])
        self.rooms.append(cur_room)
        rooms_created = 1
        start_x = int(self.max_blocks / 2)
        start_y = int(self.max_blocks / 2)
        self.set_blocks(start_x, start_y, cur_room)
        while rooms_created < room_limit:
            #Create Room
            random_rooms = self.random_rooms()
            for i in range(0, 4):
                if random_rooms[i] == "1":
                    if rooms_created >= room_limit:
                        return
                    if self.fifty_fifty() and cur_room.adj_rooms[i] == 0:
                        id = rooms_created + 1
                        new_room = self.create_room(id, room_types[id-1])
                        new_room.adj_rooms[self.get_adj_side(i)] = cur_room.id
                        cur_room_size = cur_room.size + 1
                        new_room_size = new_room.size + 1
                        valid = False
                        # Top
                        if i == 0:
                            valid = self.set_blocks(start_x, start_y - new_room_size, new_room)
                        # Right
                        elif i == 1:
                            valid = self.set_blocks(start_x + cur_room_size, start_y, new_room)
                        # Bottom
                        elif i == 2:
                            valid = self.set_blocks(start_x + cur_room_size - new_room_size, start_y + cur_room_size, new_room)
                        # Left
                        elif i == 3:
                            valid = self.set_blocks(start_x - new_room_size, start_y + cur_room_size  - new_room_size, new_room)
                        if valid:
                            self.rooms.append(new_room)
                            self.focus_queue.append(new_room.id)
                            cur_room.adj_rooms[i] = new_room.id*1
                            rooms_created += 1
                        else:
                            print("Invalid!")
            cur_room = self.get_random_focus_queue()
            if cur_room == None:
                print("Unable to find")
                return False
            start_x = cur_room.start_x
            start_y = cur_room.start_y
        return True
            

    def print_rooms(self):
        for room in self.rooms:
            print(f'Room ID {room.id}. Adj Rooms {room.adj_rooms}. Size {room.size}')
        print("-----------------")
    
    def find_room(self, id):
        for room in self.rooms:
            if room.id == id:
                return room
        return room

    def plot_room(self, img, room, start_pnt, pos):
        for i in range(0, self.max_blocks):
            for j in range(0, self.max_blocks):
                if self.blocks[i][j] != 0:
                    for m in range(0, 25):
                        for n in range(0, 25):
                            img.putpixel((i*self.block_pixel + m, j*self.block_pixel - n), self.color_dict[str(self.blocks[i][j])])

    def plot_image(self, fname):
        img = Image.new('RGB', (self.max_blocks*self.block_pixel, self.max_blocks*self.block_pixel))
        start_pnt = (self.max_blocks / 2, self.max_blocks / 2)
        self.plot_room(img, self.rooms[0], start_pnt, 0)
        img.save(fname + '.png')

class Map:

    def __init__(self):
        self.room_types = []
        self.room_types.append(RoomType(0, "Stair Room Up #1", 0))
        self.room_types.append(RoomType(1, "Stair Room Down #1", 0))
        self.room_types.append(RoomType(2, "Stair Room Up #2", 0))
        self.room_types.append(RoomType(3, "Stair Room Down #2", 0))
        self.room_types.append(RoomType(4, "Stair Room Up #3", 0))
        self.room_types.append(RoomType(5, "Stair Room Down #3", 0))
        self.room_types.append(RoomType(6, "Bathroom #1", 0))
        self.room_types.append(RoomType(7, "Bathroom #2", 0))
        self.room_types.append(RoomType(8, "Bathroom #3", 1))
        self.room_types.append(RoomType(9, "Bedroom #1", 1))
        self.room_types.append(RoomType(10 ,"Bedroom #2", 1))
        self.room_types.append(RoomType(11, "Bedroom #3", 1))
        self.room_types.append(RoomType(12, "Bedroom #4", 1))
        self.room_types.append(RoomType(13, "Bedroom #5", 2))
        self.room_types.append(RoomType(14, "Nursery", 1))
        self.room_types.append(RoomType(15, "Theater", 2))
        self.room_types.append(RoomType(16, "Music Room", 1))
        self.room_types.append(RoomType(17, "Dining Hall", 2))
        self.room_types.append(RoomType(18, "Kitchen", 2))
        self.room_types.append(RoomType(19, "Bulter Closet", 0))
        self.room_types.append(RoomType(20, "Electrical", 0))
        self.room_types.append(RoomType(21, "Office", 1))
        self.room_types.append(RoomType(22, "Gym", 1))
        self.room_types.append(RoomType(23, "Photography", 1))
        self.room_types.append(RoomType(24, "Library", 2))
        self.room_types.append(RoomType(25, "Conservatory", 1))
        self.room_types.append(RoomType(26, "Library", 2))
        self.room_types.append(RoomType(27, "Linens Room", 1))
        self.room_types.append(RoomType(28, "Ballroom", 2))
        self.room_types.append(RoomType(29, "Trophy Room", 1))
        self.floor1 = None
        self.floor2 = None
        self.floor3 = None

    def pick_rooms(self, step):
        start = 6 - (step*2)
        end = len(self.room_types)
        idx_list = [0, 1]
        out = []
        shuffle_list = []
        for i in range(start, end):
            shuffle_list.append(i)
        random.shuffle(shuffle_list)
        idx_list = idx_list + shuffle_list[0:8]
        for cur_idx in idx_list:
            out.append(self.room_types[cur_idx])
        for cur_out in out:
            self.room_types.remove(cur_out)
        random.shuffle(out)
        return out

    def generate_floor(self, num, floor_rooms):
        while True:
            floor = Floor()
            res = floor.create_rooms(num, floor_rooms)
            if res:
                return floor

    def create_blueprint(self):
        floor_rooms1 = self.pick_rooms(0)
        floor_rooms2 = self.pick_rooms(1)
        floor_rooms3 = self.pick_rooms(2)
        self.floor1 = self.generate_floor(10, floor_rooms1)
        self.floor2 = self.generate_floor(10, floor_rooms2)
        self.floor3 = self.generate_floor(10, floor_rooms3)
        self.floor1.print_rooms()
        self.floor2.print_rooms()
        self.floor3.print_rooms()
        self.floor1.plot_image("Floor 1")
        self.floor2.plot_image("Floor 2")
        self.floor3.plot_image("Floor 3")
        self.create_bin()

    def write_cells_bin(self, bin_file, cur_floor):
        end_mode = 'big'
        # Number of Blocks
        bin_file.write( cur_floor.max_blocks.to_bytes(1, end_mode) )
        # Blocks
        for i in range(0, cur_floor.max_blocks):
            for j in range(0, cur_floor.max_blocks):
                # Blocks
                bin_file.write( cur_floor.blocks[i][j].to_bytes(1, end_mode) )
        # Number of Rooms
        bin_file.write ( len(cur_floor.rooms).to_bytes(1, end_mode) )
        for room in cur_floor.rooms:
            # ID
            bin_file.write( room.id.to_bytes(1, end_mode) )
            # Room Type
            bin_file.write( room.room_type.id.to_bytes(1, end_mode))
            # Adj List
            for i in range(0, 4):
                bin_file.write(room.adj_rooms[i].to_bytes(1, end_mode))

    def create_bin(self):
        with open("map.bin", "wb") as bin_file:
            self.write_cells_bin(bin_file, self.floor1)
            self.write_cells_bin(bin_file, self.floor2)
            self.write_cells_bin(bin_file, self.floor3)
            

map = Map()
map.create_blueprint()
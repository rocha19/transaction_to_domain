// dao.ts
export interface ParketCarDAO {
  save(parkedCar: any): Promise<void>;
  update(parkedCar: any): Promise<void>;
  get(plate: string): Promise<any>;
  delete(plate: string): Promise<void>;
}

export class ParkingService {
  OPEN_HOUR = 8;
  CLOSE_HOUR = 22;
  PARKING_PRICE = 10;

  constructor(readonly parkedCarDAO: ParketCarDAO) {}

  async checkin(plate: string, checkinDate: Date) {
    if (checkinDate.getHours() < this.OPEN_HOUR)
      throw new Error("Parking lot is closed");
    if (!plate.match(/[A-Z]{3}[0-9]{4}/)) throw new Error("Invalid plate");

    const parkedCar = { plate, checkinDate };
    await this.parkedCarDAO.save(parkedCar);
  }

  async ckeckout(plate: string, checkoutDate: Date) {
    const parkedCar = await this.parkedCarDAO.get(plate);
    if (!parkedCar) throw new Error("Parked car not found");
    parkedCar.checkoutDate = checkoutDate;
    parkedCar.duration =
      (parkedCar.checkoutDate.getTime() - parkedCar.checkinDate.getTime()) /
      (1000 * 60 * 60);
    parkedCar.price = parkedCar.duration * this.PARKING_PRICE;
    return { price: parkedCar.price };
  }
}

// inMemoryDAO.ts
export class ParkedCarDAOInMemory implements ParketCarDAO {
  parkedCars: any = {};

  async save(parkedCar: any) {
    this.parkedCars[parkedCar.plate] = parkedCar;
  }

  async update(parkedCar: any) {
    this.parkedCars[parkedCar.plate] = parkedCar;
  }

  async get(plate: string) {
    return this.parkedCars[plate];
  }

  async delete(plate: string) {
    delete this.parkedCars[plate];
  }
}

// dao.spec.ts
let parketCarService: ParkingService;

beforeEath(() => {
  const parketCarDAO = new ParkedCarDAOInMemory();
  parketCarService = new ParkingService(parketCarDAO);
});

test("Deve entrar e sair do estacionamento, calculando o valor da tarifa, 10 reais por hora de permanencia", async () => {
  const plate = "AAA9999";
  const checkinDate = new Date("2024-05-29T04:00:00.000Z");
  await parketCarService.checkin(plate, checkinDate);
  const checkoutDate = new Date("2024-05-29T06:00:00.000Z");
  const ticket = await parketCarService.ckeckout(plate, checkoutDate);
  expect(ticket.price).toBe(20);
});

test("Não lançar um erro caso um carro não encontrado tente sair", async () => {
  const plate = "AAA9999";
  const checkoutDate = new Date("2024-05-29T06:00:00.000Z");
  await expect(parketCarService.ckeckout(plate, checkoutDate)).rejects.toThrow(
    new Error("Parked car not found"),
  );
});

test("Não deve entrar carro com placa inválida", async () => {
  const plate = "AAA999";
  const checkinDate = new Date("2024-05-29T04:00:00.000Z");
  await expect(parketCarService.checkin(plate, checkinDate)).rejects.toThrow(
    new Error("Invalid plate"),
  );
});

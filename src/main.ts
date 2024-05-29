export interface Clock {
  get_current_date(): Date;
}

export class ClockFake implements Clock {
  currentDate: Date = new Date();
  get_current_date(): Date {
    return this.currentDate;
  }

  setCurrentDate(date: Date) {
    this.currentDate = date;
  }
}

export default class ParkingService {
  parkedCars: Record<string, { plate: string; checkinDate: Date }> = {
    "123": { plate: "123", checkinDate: new Date() },
    "456": { plate: "456", checkinDate: new Date() },
    "789": { plate: "789", checkinDate: new Date() },
  };

  constructor() {}

  async checkin(plate: string, checkinDate: Date) {
    this.parkedCars[plate] = { plate, checkinDate };
  }

  async ckeckout(plate: string, checkoutDate: Date) {
    const parkedCar = this.parkedCars[plate];
    const duration =
      checkoutDate.getTime() -
      parkedCar.checkinDate.getTime() / (1000 * 60 * 60);
    // delete this.parkedCars[plate]
    const price = duration * 10;
    return { price };
  }
}

test("Deve entrar e sair do estacionamento, calculando o valor da tarifa, 10 reais por hora de permanencia", async () => {
  const parkingService = new ParkingService();
  const plate = "AAA9999";
  const checkinDate = new Date("2024-05-29T04:00:00.000Z");
  await parkingService.checkin(plate, checkinDate);
  const checkoutDate = new Date("2024-05-29T06:00:00.000Z");
  const ticket = parkingService.ckeckout(plate, checkoutDate);
  expect(ticket.price).toBe(20);
});

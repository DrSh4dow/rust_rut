pub struct Rut {
    rut_len: u32,
    rut_as_u32: u32,
}

impl Rut {
    pub fn new(args: &[String]) -> Result<Rut, &str> {
        if args.len() < 2 {
            return Err("Cantidad de argumentos insuficiente");
        }

        let rut_len: u32 = match args[1].trim().len().try_into() {
            Ok(num) => num,
            Err(_) => return Err("No ingreso un valor valido"),
        };

        if rut_len < 7 || rut_len > 12 {
            return Err("El rut no tiene un largo valido");
        }

        let rut_as_u32: u32 = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("No ingreso un numero valido."),
        };

        Ok(Rut {
            rut_len,
            rut_as_u32,
        })
    }

    pub fn calcular_digito_verificador(&self) -> String {
        let mut suma: u32 = 0;
        let mut i: u32 = 0;
        let mut factor: u32 = 2;
        while i < self.rut_len {
            let numero = (self.rut_as_u32 / 10_u32.pow(i)) % 10;
            suma += numero * factor;

            i += 1;
            factor += 1;
            if factor == 8 {
                factor = 2;
            }
        }
        let digito_verificador = 11 - (suma % 11);

        if digito_verificador == 11 {
            "0".to_owned()
        } else if digito_verificador == 10 {
            "K".to_owned()
        } else {
            digito_verificador.to_string()
        }
    }
}

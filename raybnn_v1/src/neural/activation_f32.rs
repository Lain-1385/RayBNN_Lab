extern crate arrayfire;

const zero: f32 = 0.0;
const high: f32 = 1000000.0;
const two: f32 = 2.0;

pub fn ReLU(X: &arrayfire::Array<f32>) -> arrayfire::Array<f32> {
    arrayfire::clamp(X, &zero, &high, false)
}

pub fn Softplus(X: &arrayfire::Array<f32>) -> arrayfire::Array<f32> {
    let mut temp = -arrayfire::abs(X);
    temp = arrayfire::exp(&temp);
    ReLU(X) + arrayfire::log1p(&temp)
}

/*
pub fn UAF(
    X: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>) ->  arrayfire::Array<f32> {


    // X + B
    let mut temp0 = arrayfire::add(X, B, true);
    // X^2
    let mut temp1 = arrayfire::pow(X,&two,false);

    //A(X + B)  +  C( X^2 )
    temp0 = arrayfire::mul(A,&temp0,true) + arrayfire::mul(C,&temp1,true);


    // X - B;
    temp1 = arrayfire::sub(X, B, true);
    // D (X - B)
    temp1 = arrayfire::mul(D,&temp1,true);


    //Softplus( A(X + B)  +  C( X^2 ) )
    temp0 = Softplus(&temp0);
    //Softplus( D (X - B) )
    temp1 = Softplus(&temp1);

    //  Softplus( A(X + B)  +  C( X^2 ) ) - Softplus( D (X - B) )
    temp0 = temp0 - temp1;

    // Softplus( A(X + B)  +  C( X^2 ) ) - Softplus( D (X - B) ) + E
    arrayfire::add(&temp0, E, true)
}

*/

pub fn UAF(
    X: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>,
) -> arrayfire::Array<f32> {
    // X + B
    let mut temp0 = arrayfire::add(X, B, true);
    // X^2
    let mut temp1 = arrayfire::pow(X, &two, false);

    // -|C|
    let mut temp2 = -arrayfire::abs(C);

    //A(X + B)  +  -|C|( X^2 )
    temp0 = arrayfire::mul(A, &temp0, true) + arrayfire::mul(&temp2, &temp1, true);

    drop(temp2);

    // X - B;
    temp1 = arrayfire::sub(X, B, true);
    // D (X - B)
    temp1 = arrayfire::mul(D, &temp1, true);

    //Softplus( A(X + B)  +  C( X^2 ) )
    temp0 = Softplus(&temp0);
    //Softplus( D (X - B) )
    temp1 = Softplus(&temp1);

    //  Softplus( A(X + B)  +  C( X^2 ) ) - Softplus( D (X - B) )
    temp0 = temp0 - temp1;

    // Softplus( A(X + B)  +  C( X^2 ) ) - Softplus( D (X - B) ) + E
    arrayfire::add(&temp0, E, true)
}

/*

pub fn deriUAF(
    X: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>,
    dX: &mut arrayfire::Array<f32>,
    dA: &mut arrayfire::Array<f32>,
    dB: &mut arrayfire::Array<f32>,
    dC: &mut arrayfire::Array<f32>,
    dD: &mut arrayfire::Array<f32>,
    dE: &mut arrayfire::Array<f32>)
{

    // X + B
    let mut temp0 = arrayfire::add(X, B, true);
    // X^2
    let mut temp1 = arrayfire::pow(X,&two,false);
    //A(X + B)  +  C( X^2 )
    let mut expcal0 = arrayfire::mul(A,&temp0,true) + arrayfire::mul(C,&temp1,true);

    //Sigmoid( A(X + B)  +  C( X^2 ) )
    expcal0 = arrayfire::sigmoid(&expcal0);

    //dA = (X + B) Sigmoid( A(X + B)  +  C( X^2 ) )
    *dA = arrayfire::mul(&temp0,&expcal0,false);
    //dB = (A) Sigmoid( A(X + B)  +  C( X^2 ) )
    *dB = arrayfire::mul(A,&expcal0,true);
    //dC = (X^2) Sigmoid( A(X + B)  +  C( X^2 ) )
    *dC = arrayfire::mul(&temp1,&expcal0,false);
    //  *dC = 0.0f32*arrayfire::mul(&temp1,&expcal0,false);



    //A + 2Cx
    temp0 = two*arrayfire::mul(C,X,true);
    temp0 = arrayfire::add(A, &temp0, true);
    // (A + 2Cx) Sigmoid( A(X + B)  +  C( X^2 ) )
    expcal0 = arrayfire::mul(&temp0,&expcal0,false);

    // X - B
    temp0 = arrayfire::sub(X, B, true);
    //D ( X - B )
    let mut expcal1 = arrayfire::mul(D,&temp0,true);

    //Sigmoid( D ( X - B )  )
    expcal1 = arrayfire::sigmoid(&expcal1);

    *dB = dB.clone() + arrayfire::mul(D,&expcal1 ,true);

    //dD = - (X - B) Sigmoid( D ( X - B )  )
    *dD = -arrayfire::mul(&temp0,&expcal1,false);
    //dE = 1
    *dE = arrayfire::constant::<f32>(1.0, X.dims());

    expcal1 = arrayfire::mul(D,&expcal1,true);

    //dX = (A + 2Cx) Sigmoid( A(X + B)  +  C( X^2 ) ) - D Sigmoid( D ( X - B )  )
    *dX = (expcal0-expcal1);
}

*/

pub fn deriUAF(
    X: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>,
    dX: &mut arrayfire::Array<f32>,
    dA: &mut arrayfire::Array<f32>,
    dB: &mut arrayfire::Array<f32>,
    dC: &mut arrayfire::Array<f32>,
    dD: &mut arrayfire::Array<f32>,
    dE: &mut arrayfire::Array<f32>,
) {
    // X + B
    let mut temp0 = arrayfire::add(X, B, true);
    // X^2
    let mut temp1 = arrayfire::pow(X, &two, false);

    // -|C|
    let mut temp2 = -arrayfire::abs(C);

    //A(X + B)   -  |C|( X^2 )
    let mut expcal0 = arrayfire::mul(A, &temp0, true) + arrayfire::mul(&temp2, &temp1, true);

    //Sigmoid( A(X + B)   -  |C|( X^2 ) )
    expcal0 = arrayfire::sigmoid(&expcal0);

    //dA = (X + B) Sigmoid( A(X + B)   -  |C|( X^2 ) )
    *dA = arrayfire::mul(&temp0, &expcal0, false);
    //dB = (A) Sigmoid( A(X + B)   -  |C|( X^2 ) )
    *dB = arrayfire::mul(A, &expcal0, true);

    // -sign(C)
    temp2 = 2.0f32 * (arrayfire::sign(C) - 0.5f32);

    // -sign(C) (X^2)
    temp1 = arrayfire::mul(&temp2, &temp1, true);

    //dC = -sign(C)(X^2) Sigmoid( A(X + B)  +  -|C|( X^2 ) )
    *dC = arrayfire::mul(&temp1, &expcal0, false);

    // -|C|
    temp2 = -arrayfire::abs(C);

    //A - 2|C|x
    temp0 = two * arrayfire::mul(&temp2, X, true);
    temp0 = arrayfire::add(A, &temp0, true);
    // (A - 2|C|x) Sigmoid( A(X + B)  -  |C|( X^2 ) )
    expcal0 = arrayfire::mul(&temp0, &expcal0, false);

    // X - B
    temp0 = arrayfire::sub(X, B, true);
    //D ( X - B )
    let mut expcal1 = arrayfire::mul(D, &temp0, true);

    //Sigmoid( D ( X - B )  )
    expcal1 = arrayfire::sigmoid(&expcal1);

    *dB = dB.clone() + arrayfire::mul(D, &expcal1, true);

    //dD = - (X - B) Sigmoid( D ( X - B )  )
    *dD = -arrayfire::mul(&temp0, &expcal1, false);
    //dE = 1
    *dE = arrayfire::constant::<f32>(1.0, X.dims());

    expcal1 = arrayfire::mul(D, &expcal1, true);

    //dX = (A - 2|C|x) Sigmoid( A(X + B)  - |C|( X^2 ) ) - D Sigmoid( D ( X - B )  )
    *dX = (expcal0 - expcal1);
}

extern crate arrayfire;

const ODE45_C2: f32 = 1.0 / 5.0;
const ODE45_A21: f32 = 1.0 / 5.0;

const ODE45_C3: f32 = 3.0 / 10.0;
const ODE45_A31: f32 = 3.0 / 40.0;
const ODE45_A32: f32 = 9.0 / 40.0;

const ODE45_C4: f32 = 4.0 / 5.0;
const ODE45_A41: f32 = 44.0 / 45.0;
const ODE45_A42: f32 = -56.0 / 15.0;
const ODE45_A43: f32 = 32.0 / 9.0;

const ODE45_C5: f32 = 8.0 / 9.0;
const ODE45_A51: f32 = 19372.0 / 6561.0;
const ODE45_A52: f32 = -25360.0 / 2187.0;
const ODE45_A53: f32 = 64448.0 / 6561.0;
const ODE45_A54: f32 = -212.0 / 729.0;

const ODE45_C6: f32 = 1.0;
const ODE45_A61: f32 = 9017.0 / 3168.0;
const ODE45_A62: f32 = -355.0 / 33.0;
const ODE45_A63: f32 = 46732.0 / 5247.0;
const ODE45_A64: f32 = 49.0 / 176.0;
const ODE45_A65: f32 = -5103.0 / 18656.0;

const ODE45_C7: f32 = 1.0;
//const ODE45_A71: f32 = 35.0/384.0;
//const ODE45_A72: f32 = 0.0;
//const ODE45_A73: f32 = 500.0/1113.0;
//const ODE45_A74: f32 = 125.0/192.0;
//const ODE45_A75: f32 = -2187.0/6784.0;
//const ODE45_A76: f32 = 11.0/84.0;

const ODE45_B1: f32 = 35.0 / 384.0;
//const ODE45_B2: f32 = 0.0;
const ODE45_B3: f32 = 500.0 / 1113.0;
const ODE45_B4: f32 = 125.0 / 192.0;
const ODE45_B5: f32 = -2187.0 / 6784.0;
const ODE45_B6: f32 = 11.0 / 84.0;
//const ODE45_B7: f32 = 0.0;

const ODE45_B1E: f32 = 5179.0 / 57600.0;
//const ODE45_B2E: f32 = 0.0;
const ODE45_B3E: f32 = 7571.0 / 16695.0;
const ODE45_B4E: f32 = 393.0 / 640.0;
const ODE45_B5E: f32 = -92097.0 / 339200.0;
const ODE45_B6E: f32 = 187.0 / 2100.0;
const ODE45_B7E: f32 = 1.0 / 40.0;

use crate::neural::network_f32::network_metadata_type;

use crate::diffeq::ode45_f32::ode45_f32_set;

use crate::neural::activation_f32::UAF;

pub fn state_space_grad(
    X: &arrayfire::Array<f32>,
    W: &arrayfire::Array<f32>,
    H: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>,
    Z: &mut arrayfire::Array<f32>,
    Q: &mut arrayfire::Array<f32>,
) {
    let input_size = X.dims()[0];
    let seqs = &[
        arrayfire::Seq::new(0.0f32, (input_size - 1) as f32, 1.0f32),
        arrayfire::Seq::default(),
    ];

    //Add X to Z
    arrayfire::assign_seq(Z, seqs, X);

    *Z = arrayfire::matmul(W, Z, arrayfire::MatProp::NONE, arrayfire::MatProp::NONE);
    *Z = arrayfire::add(Z, H, true);

    *Q = UAF(Z, A, B, C, D, E);
}

/*
pub fn state_space_forward_solve(
    ODE_opts: &ode45_f32_set,
    neural_opts: &network_metadata_type,


    in_t: &arrayfire::Array<f32>,
    X: &arrayfire::Array<f32>,
    dXdt: &arrayfire::Array<f32>,


    W: &arrayfire::Array<f32>,
    H: &arrayfire::Array<f32>,
    A: &arrayfire::Array<f32>,
    B: &arrayfire::Array<f32>,
    C: &arrayfire::Array<f32>,
    D: &arrayfire::Array<f32>,
    E: &arrayfire::Array<f32>,



    out_t: &mut arrayfire::Array<f32>,
    Y: &mut arrayfire::Array<f32>,
    dYdt: &mut arrayfire::Array<f32>,
    dYdt_UAF: &mut arrayfire::Array<f32>

    )
    {

    //let var_num: u64 = initial.dims()[1];
    let batch_size = X.dims()[1];


    let neuron_size: u64 = neural_opts.neuron_size.clone();
    let input_size: u64 = neural_opts.input_size.clone();
    let output_size: u64 = neural_opts.output_size.clone();



    let t_dims = arrayfire::Dim4::new(&[1,1,1,1]);


    let mut t: f32 = ODE_opts.tstart.clone()  ;
    let tend: f32 =   ODE_opts.tend.clone()  ;
    let mut tstep: f32 =  ODE_opts.tstep.clone() ;
    let rtol: f32 = ODE_opts.rtol.clone() ;
    let atol: f32 = ODE_opts.atol.clone() ;
    let normctrl: bool = ODE_opts.normctrl.clone() ;
    let mut cur_point = arrayfire::slice(X, 0);


    //Calculate derivative k1
    let mut k1_Z = arrayfire::constant::<f32>(0.0,t_dims);
    let mut k1 = arrayfire::constant::<f32>(0.0,t_dims);


    state_space_grad(
        &cur_point,
        W,
        H,
        A,
        B,
        C,
        D,
        E,
        &mut k1_Z,
        &mut k1
    );

    let Y_dims = arrayfire::Dim4::new(&[output_size,batch_size,1,1]);

    //Output array
    *out_t = arrayfire::constant::<f32>(0.0,t_dims);
    *Y = arrayfire::constant::<f32>(0.0,Y_dims);
    *dYdt = arrayfire::constant::<f32>(0.0,Y_dims);
    *dYdt_UAF = arrayfire::constant::<f32>(0.0,Y_dims);


    let mut nerr: f32 = 1.0;
    let mut rerr: f32 = 1.0;
    let mut tol: f32 = 1.0;

    let cmp_dims = arrayfire::Dim4::new(&[2,var_num,1,1]);
    let mut cmparr = arrayfire::constant::<f32>(t,t_dims);

    if normctrl == false
    {
        cmparr = arrayfire::constant::<f32>(atol,cmp_dims);
    }


    let mut tol_cpu: Vec<f32> = vec![1.0];
    let mut nerr_cpu: Vec<f32> = vec![1.0];

    //let firstseq = &[arrayfire::Seq::default(), arrayfire::Seq::new(0.0, 0.0, 1.0)];



    let mut t2 = t.clone();
    let mut t3 = t.clone();
    let mut t4 = t.clone();
    let mut t5 = t.clone();
    let mut t6 = t.clone();
    let mut t7 = t.clone();



    let mut point2 = cur_point.clone();
    let mut point3 = cur_point.clone();
    let mut point4 = cur_point.clone();
    let mut point5 = cur_point.clone();
    let mut point6 = cur_point.clone();
    let mut point7 = cur_point.clone();




    let mut k2 = k1.clone();
    let mut k3 = k1.clone();
    let mut k4 = k1.clone();
    let mut k5 = k1.clone();
    let mut k6 = k1.clone();
    let mut k7 = k1.clone();


    let mut y0 = k1.clone();
    let mut y1 = k1.clone();
    let mut subtract = k1.clone();


    let mut t_elem = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut abserror = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut absvec = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut minarr = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut result = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut tol_gpu = arrayfire::constant::<f32>(t.clone() ,t_dims);
    let mut nerr_gpu = arrayfire::constant::<f32>(t.clone() ,t_dims);







    while   t < tend  {

        //Time arr 2
        t2 = t.clone() + (tstep*ODE45_C2) ;

        //Create point vector 2
        point2 = cur_point.clone() +  (tstep*ODE45_A21*(k1.clone()) );

        //Calculate derivative k2
        k2 = diffeq(t2, &point2);






        //Time arr 3
        t3 = t.clone()  + (tstep*ODE45_C3) ;

        //Create point vector 3
        point3 = cur_point.clone() +  tstep*(  (ODE45_A31*(k1.clone()))   +  (ODE45_A32*(k2.clone()))    );

        //Calculate derivative k3
        k3 = diffeq(t3,&point3) ;







        //Time arr 4
        t4 = t.clone() + (tstep*ODE45_C4);

        //Create point vector 4
        point4 = cur_point.clone() +  tstep*(  (ODE45_A41*(k1.clone()))   +  (ODE45_A42*(k2.clone()))  +  (ODE45_A43*(k3.clone()))   );


        //Calculate derivative k4
        k4 = diffeq(t4,&point4);






        //Time arr 5
        t5 = t.clone() + (tstep*ODE45_C5) ;

        //Create point vector 4
        point5 = cur_point.clone() +  tstep*(  (ODE45_A51*(k1.clone()))   +  (ODE45_A52*(k2.clone()))  +  (ODE45_A53*(k3.clone()))      +  (ODE45_A54*(k4.clone()))      );

        //Calculate derivative k5
        k5 = diffeq(t5,&point5);








        //Time arr 5
        t6 = t.clone() + (tstep*ODE45_C6);

        //Create point vector 4
        point6 = cur_point.clone() +  tstep*(  (ODE45_A61*(k1.clone()))   +  (ODE45_A62*(k2.clone()))  +  (ODE45_A63*(k3.clone()))      +  (ODE45_A64*(k4.clone()))   +  (ODE45_A65*(k5.clone()))     );

        //Calculate derivative k4
        k6 = diffeq(t6,&point6);






        //Time arr 5
        t7 = t.clone() + (tstep*ODE45_C7);

        //Create point vector 4
        //let point7 = cur_point.clone() +  tstep*(  (ODE45_A71*(k1.clone()))   +  (ODE45_A72*(k2.clone()))  +  (ODE45_A73*(k3.clone()))      +  (ODE45_A74*(k4.clone()))   +  (ODE45_A75*(k5.clone()))    +  (ODE45_A76*(k6.clone()))   );

        y0 = tstep*( (ODE45_B1*k1.clone())  +   (ODE45_B3*k3.clone()) +  (ODE45_B4*k4.clone()) +  (ODE45_B5*k5.clone()) +  (ODE45_B6*k6.clone())  );
        point7 = cur_point.clone() + y0.clone();

        //Calculate derivative k4
        k7 = diffeq(t7,&point7);


        //Update point
        //let y0 = tstep*( (ODE45_B1*k1.clone())  +  (ODE45_B2*k2.clone()) +  (ODE45_B3*k3.clone()) +  (ODE45_B4*k4.clone()) +  (ODE45_B5*k5.clone()) +  (ODE45_B6*k6.clone()) +  (ODE45_B7*k7.clone()) );
        //let y1 = tstep*( (ODE45_B1E*k1.clone())  +  (ODE45_B2E*k2) +  (ODE45_B3E*k3) +  (ODE45_B4E*k4) +  (ODE45_B5E*k5) +  (ODE45_B6E*k6) +  (ODE45_B7E*k7) );
        //let subtract = y1.clone() - y0.clone();


        y1 = tstep*( (ODE45_B1E*k1.clone())   +  (ODE45_B3E*k3) +  (ODE45_B4E*k4) +  (ODE45_B5E*k5) +  (ODE45_B6E*k6) +  (ODE45_B7E*k7.clone()) );
        subtract = y1.clone() - y0.clone();


        if normctrl
        {
            nerr = arrayfire::norm::<f32>(&subtract,arrayfire::NormType::VECTOR_2,0.0,0.0  ) as f32  ;
            rerr = arrayfire::norm::<f32>(&y0,arrayfire::NormType::VECTOR_2,0.0,0.0  )  as f32;
            tol = atol.min( rtol*rerr );
        }
        else
        {
            abserror = arrayfire::abs(&subtract);
            absvec = rtol * arrayfire::abs(&y0);

            arrayfire::set_row(&mut cmparr, &absvec,1);
            minarr = arrayfire::min(&cmparr,0);
            result = abserror.clone() - minarr.clone();


            let (_,_,idx) = arrayfire::imax_all(&result);

            tol_gpu = arrayfire::col(&minarr, idx as i64);
            nerr_gpu = arrayfire::col(&abserror, idx as i64);



            //let (_,idx) = arrayfire::imax(&result,1);
            //let idx = arrayfire::index(&idx, firstseq);

            //let mut idxer0 =  arrayfire::Indexer::default();
            //idxer0.set_index(&idx, 0, None);

            //tol_gpu =  arrayfire::index_gen(&minarr, idxer0);

            tol_gpu.host(&mut tol_cpu);
            tol = tol_cpu[0];

            //let mut idxer1 =  arrayfire::Indexer::default();
            //idxer1.set_index(&idx, 0, None);

            //nerr_gpu = arrayfire::index_gen(&abserror, idxer1);
            nerr_gpu.host(&mut nerr_cpu);
            nerr = nerr_cpu[0];
        }




        if  nerr < tol
        {
            //New point
            cur_point = point7.clone();

            //New time
            t = t7.clone();

            //New derivative
            k1 = k7.clone();

            t_elem = arrayfire::constant::<f32>(t.clone() ,t_dims);

            //Save to array
            *out_t_arr = arrayfire::join::<f32>(0,out_t_arr,&t_elem);

            *out_f_arr = arrayfire::join::<f32>(0,out_f_arr,&cur_point);

            *out_dfdt_arr = arrayfire::join::<f32>(0,out_dfdt_arr,&k1);

        }


        tstep = 0.9*tstep*( ( ( (tol/(nerr + 1E-30)).powf(0.2)).max(0.1)  ).min(10.0)  );

    }


}
*/

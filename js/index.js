
async function run_render() {
    const lib = await import("../pkg/index.js").catch(console.error);
    const start = Date.now();
    const scale = $('#scale').val(); 
    const x_r = $('#startReal').val();
    const x_c = $('#startComplex').val();
    const red_g = $('#redGrad').val();
    const blue_g = $('#blueGrad').val();
    lib.render_fractal(scale, x_r, x_c, red_g, blue_g);
    const end = Date.now();
    $("#stats").text(`Execution time: ${end - start} ms`);

}

async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    $('#scale').val(3.0); 
    $('#startReal').val(-0.4);
    $('#startComplex').val(0.6);
    $('#redGrad').val(0.2);
    $('#blueGrad').val(0.2);
    run_render()
    $(document).ready(function() {
        console.log("Doc ready")
        $('#paramsSubmit').on('click', function(e) {
            run_render();
            $('#offcanvas').offcanvas('hide');
            e.preventDefault();
        })
     });

    
}

main();

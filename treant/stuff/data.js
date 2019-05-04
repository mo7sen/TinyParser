var datajson ;

$.getJSON("http://localhost:3000/tree/get",function( data ) {
    console.log(data);   
    datajson = data;
    console.log(datajson);
    do_dat_shit();
  });

function do_dat_shit(){
simple_chart_config = {
  chart: {
    container: "#tree-simple",
    // hideRootNode: true,
    connectors: {
      type: "bCurve"
    },
    node: {
      collapsable: true
    },
    animation: {
      nodeAnimation: "easeInOutSine",
      nodeSpeed: 600,
      connectorsAnimation: "easeInOutSine",
      connectorsSpeed: 300
    }
  },
  nodeStructure: datajson
};
new Treant(simple_chart_config, function() {
  alert("Tree Loaded");
});
}
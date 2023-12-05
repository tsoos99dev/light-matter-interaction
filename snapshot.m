data = readmatrix("out/sim.csv");

L = size(data, 2);
X = 0:L-1;

figure;
aspect = [16 9];
f = gcf;
f.Position(3:4) = aspect / aspect(1) * 1200;
pbaspect([aspect 1])
set(gcf,'color','w');
set(gca, 'FontName', 'Helvetica');
xlabel('x $[\mathrm{nm}]$', 'Interpreter', 'latex', 'FontSize', 16)
ylabel('E $[\mathrm{\frac{V}{m}}]$', 'Interpreter', 'latex', 'FontSize', 16)
set(gca, 'Box', 'off', 'TickDir', 'out', 'TickLength', [.02 .02], ...
    'XMinorTick', 'on', 'YMinorTick', 'on', 'YGrid', 'on', ...
    'XColor', [.3 .3 .3], 'YColor', [.3 .3 .3], ...
    'LineWidth', 1)

plot(X,data, 'Color',"#FF5714", 'LineWidth', 2);



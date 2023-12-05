data = readmatrix("out/sim.csv");

T = size(data, 1);
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

a1 = animatedline('Color',"#FF5714", "LineWidth", 2);
axis([0 L -1.5 1.5])

hold on;
xline(100, 'LineWidth', 2);

for k = 1:T
    yk = data(k, :);
    clearpoints(a1);
    addpoints(a1, X, yk);
    drawnow limitrate
    pause(0.03);
end
drawnow